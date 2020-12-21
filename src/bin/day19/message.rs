#[derive(Debug, Eq, PartialEq)]
pub struct RuleSet {
    rules: Vec<Vec<Rule>>,
}

#[derive(Debug, Eq, PartialEq)]
enum Rule {
    Literal(char),
    SubRules(Vec<usize>),
}

impl From<String> for RuleSet {
    fn from(rule_list: String) -> Self {
        let mut rules = Vec::new();

        rule_list.split_terminator('\n')
            .map(|line| Rule::parse_line(line))
            .for_each(|(id, parsed_rules)| {
                rules.insert(id, parsed_rules);
            });

        RuleSet {
            rules
        }
    }
}

impl Rule {
    fn parse_line(line: &str) -> (usize, Vec<Self>) {
        let id_terminator = line.find(':').unwrap();
        let id: usize = line[0..id_terminator].parse().unwrap();

        let rules = if let Some(opening_quote) = line.find('"') {
            vec![Rule::Literal(line[(opening_quote) + 1..].chars().next().unwrap())]
        } else {
            line[(id_terminator + 1)..].split('|')
                .map(|block| {
                    Rule::SubRules(block.split(' ')
                        .map(|sub_rule| sub_rule.trim())
                        .filter(|sub_rule| !sub_rule.is_empty())
                        .map(|sub_rule| sub_rule.parse().unwrap())
                        .collect())
                })
                .collect()
        };

        (id, rules)
    }
}

#[cfg(test)]
mod test {
    use crate::message::{Rule, RuleSet};

    #[test]
    fn rule_from_string() {
        assert_eq!((0, vec![Rule::SubRules(vec![1, 2])]), Rule::parse_line("0: 1 2"));
        assert_eq!((1, vec![Rule::Literal('a')]), Rule::parse_line("1: \"a\""));
        assert_eq!((2, vec![Rule::SubRules(vec![1, 3]), Rule::SubRules(vec![3, 1])]), Rule::parse_line("2: 1 3 | 3 1"));
        assert_eq!((3, vec![Rule::Literal('b')]), Rule::parse_line("3: \"b\""));
    }

    #[test]
    fn rule_set_from_string() {
        let rules = String::from("0: 1 2\n\
                           1: \"a\"\n\
                           2: 1 3 | 3 1\n\
                           3: \"b\"\n");

        let expected = RuleSet {
            rules: vec![
                vec![Rule::SubRules(vec![1, 2])],
                vec![Rule::Literal('a')],
                vec![Rule::SubRules(vec![1, 3]), Rule::SubRules(vec![3, 1])],
                vec![Rule::Literal('b')],
            ]
        };

        assert_eq!(expected, RuleSet::from(rules));
    }
}