use std::collections::HashMap;
use std::iter::FromIterator;

use itertools::Itertools;

#[derive(Debug, Eq, PartialEq)]
pub struct BagRules {
    rules: HashMap<String, Vec<(String, u32)>>
}

impl <'a> FromIterator<&'a str> for BagRules {

    fn from_iter<T: IntoIterator<Item=&'a str>>(lines: T) -> Self {
        let rules: HashMap<String, Vec<(String, u32)>> = lines.into_iter()
            .map(|line| {
                let (container, contents) = line.split(" bags contain ").collect_tuple().unwrap();

                let targets: Vec<(String, u32)> = contents.split(", ")
                    .map(|rule| {
                        let first_space = rule.find(" ").unwrap();
                        let _start_of_bag = rule.find(" bag").unwrap();

                        let quantity: u32 = rule[0..first_space].parse().unwrap();
                        let target = &rule[(first_space + 1).._start_of_bag];

                        (String::from(target), quantity)
                    })
                    .collect();

                (String::from(container), targets)
            })
            .collect();

        BagRules {
            rules
        }
    }
}

#[cfg(test)]
mod test {
    use crate::bags::BagRules;
    use std::collections::HashMap;

    #[test]
    fn load() {
        let mut expected_rules = HashMap::new();
        expected_rules.insert(String::from("light red"),
                              vec![(String::from("bright white"), 1), (String::from("muted yellow"), 2)]);

        let expected = BagRules {
            rules: expected_rules
        };

        let rules: BagRules = vec!["light red bags contain 1 bright white bag, 2 muted yellow bags."].into_iter().collect();

        assert_eq!(expected, rules);
    }
}