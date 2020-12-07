use std::collections::HashMap;

pub fn parse(passport: String) -> HashMap<String, String> {
    let mut parsed = HashMap::new();

    passport.replace("\n", " ").split(" ")
        .filter(|pair| !pair.trim().is_empty())
        .for_each(|pair| {
            let pieces:Vec<_> = pair.split(':').collect();

            if pieces.len() == 2 {
                parsed.insert(pieces[0].to_string(), pieces[1].to_string());
            }
        });

    parsed
}

pub fn is_valid(passport: &HashMap<String, String>, required_fields: &[String]) -> bool {
    for required_field in required_fields.iter() {
        if !passport.contains_key(required_field) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use crate::passport;
    use std::collections::HashMap;

    #[test]
    fn parse() {
        let passport = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm".to_string();

        let mut expected = HashMap::new();
        expected.insert("ecl".to_string(), "gry".to_string());
        expected.insert("pid".to_string(), "860033327".to_string());
        expected.insert("eyr".to_string(), "2020".to_string());
        expected.insert("hcl".to_string(), "#fffffd".to_string());
        expected.insert("byr".to_string(), "1937".to_string());
        expected.insert("iyr".to_string(), "2017".to_string());
        expected.insert("cid".to_string(), "147".to_string());
        expected.insert("hgt".to_string(), "183cm".to_string());

        assert_eq!(expected, passport::parse(passport));
    }

    #[test]
    fn is_valid() {
        let required_fields = [
            "byr".to_string(),
            "iyr".to_string(),
            "eyr".to_string(),
            "hgt".to_string(),
            "hcl".to_string(),
            "ecl".to_string(),
            "pid".to_string(),
        ];

        {
            let passport = passport::parse("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm".to_string());
            assert!(passport::is_valid(&passport, &required_fields));
        }

        {
            let passport = passport::parse("pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm".to_string());
            assert!(!passport::is_valid(&passport, &required_fields));
        }
    }
}