use std::collections::{HashMap, VecDeque, HashSet};
use std::iter::FromIterator;

use itertools::Itertools;

#[derive(Debug, Eq, PartialEq)]
pub struct BagRules {
    rules: HashMap<String, Vec<(String, u32)>>
}

impl BagRules {
    pub fn get_top_level_containers(&self, target: &String) -> usize {
        let mut containers_with_path = 0;

        self.rules.keys()
            .for_each(|container| {
                if self.has_path(container, target) {
                    containers_with_path += 1;
                }
            });

        containers_with_path
    }

    fn has_path(&self, origin: &String, target: &String) -> bool {
        // TODO We could make this all more efficient by returning everything on the path and
        // memoizing.
        let mut stack = VecDeque::new();
        let mut visited = HashSet::new();

        stack.push_front(origin);

        while !stack.is_empty() {
            let bag = stack.pop_front().unwrap();

            if !visited.contains(bag) {
                visited.insert(bag);

                for (contained_bag, _quantity) in self.rules.get(bag).unwrap() {
                    if contained_bag == target {
                        return true;
                    }

                    stack.push_front(contained_bag);
                }
            }
        }

        false
    }
}

impl FromIterator<String> for BagRules {

    fn from_iter<T: IntoIterator<Item=String>>(lines: T) -> Self {
        let rules: HashMap<String, Vec<(String, u32)>> = lines.into_iter()
            .map(|line| {
                let (container, contents) = line.split(" bags contain ").collect_tuple().unwrap();

                let targets: Vec<(String, u32)> = contents.split(", ")
                    .filter(|rule| rule != &"no other bags.")
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

    fn has_path() {
        let rules: BagRules = vec!["light red bags contain 1 bright white bag, 2 muted yellow bags.",
                                   "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
                                   "bright white bags contain 1 shiny gold bag.",
                                   "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
                                   "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
                                   "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
                                   "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
                                   "faded blue bags contain no other bags.",
                                   "dotted black bags contain no other bags.",].into_iter().collect();

        assert_eq!(true, rules.has_path(&String::from("light red"), &String::from("shiny gold")));
        assert_eq!(true, rules.has_path(&String::from("dark orange"), &String::from("shiny gold")));
        assert_eq!(true, rules.has_path(&String::from("bright white"), &String::from("shiny gold")));
        assert_eq!(true, rules.has_path(&String::from("muted yellow"), &String::from("shiny gold")));
        assert_eq!(false, rules.has_path(&String::from("shiny gold"), &String::from("shiny gold")));
        assert_eq!(false, rules.has_path(&String::from("dark olive"), &String::from("shiny gold")));
        assert_eq!(false, rules.has_path(&String::from("vibrant plum"), &String::from("shiny gold")));
        assert_eq!(false, rules.has_path(&String::from("faded blue"), &String::from("shiny gold")));
        assert_eq!(false, rules.has_path(&String::from("dotted black"), &String::from("shiny gold")));
    }

    fn get_top_level_containers() {
        let rules: BagRules = vec!["light red bags contain 1 bright white bag, 2 muted yellow bags.",
                                   "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
                                   "bright white bags contain 1 shiny gold bag.",
                                   "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
                                   "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
                                   "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
                                   "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
                                   "faded blue bags contain no other bags.",
                                   "dotted black bags contain no other bags.",].into_iter().collect();

        assert_eq!(4, rules.get_top_level_containers(&String::from("shiny gold")));
    }
}