use std::collections::HashSet;

lazy_static! {
    static ref ALL_ANSWERS: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
}

#[derive(Debug)]
pub struct FormGroup {
    responses: Vec<HashSet<char>>
}

impl FormGroup {
    pub fn from_answers(answers: &str) -> FormGroup {
        let responses = answers.split('\n')
            .map(|row| {
                row.chars().collect()
            })
            .collect();

        FormGroup { responses }
    }

    pub fn get_distinct_answers(&self) -> HashSet<char> {
        self.responses.iter().fold(HashSet::new(), |a, b| &a | b)
    }

    pub fn get_universal_answers(&self) -> HashSet<char> {
        self.responses.iter().fold(ALL_ANSWERS.clone(), |a, b| &a & b)
    }
}

#[cfg(test)]
mod test {
    use crate::customs::FormGroup;
    use std::collections::HashSet;

    #[test]
    fn get_distinct_answers() {
        let group = FormGroup::from_answers("abc\ncde");
        let expected: HashSet<char> = "abcde".chars().collect();

        assert_eq!(expected, group.get_distinct_answers());
    }

    #[test]
    fn get_universal_answers() {
        let group = FormGroup::from_answers("abc\ncde");
        let expected: HashSet<char> = "c".chars().collect();

        assert_eq!(expected, group.get_universal_answers());
    }
}