use std::collections::HashMap;

lazy_static! {
    static ref REQUIRED_FIELDS:[String;7] = [
        String::from("byr"),
        String::from("iyr"),
        String::from("eyr"),
        String::from("hgt"),
        String::from("hcl"),
        String::from("ecl"),
        String::from("pid"),
    ];
}

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

pub fn has_required_fields(passport: &HashMap<String, String>) -> bool {
    for required_field in REQUIRED_FIELDS.iter() {
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
        expected.insert(String::from("ecl"), String::from("gry"));
        expected.insert(String::from("pid"), String::from("860033327"));
        expected.insert(String::from("eyr"), String::from("2020"));
        expected.insert(String::from("hcl"), String::from("#fffffd"));
        expected.insert(String::from("byr"), String::from("1937"));
        expected.insert(String::from("iyr"), String::from("2017"));
        expected.insert(String::from("cid"), String::from("147"));
        expected.insert(String::from("hgt"), String::from("183cm"));

        assert_eq!(expected, passport::parse(passport));
    }

    #[test]
    fn has_required_fields() {
        {
            let passport = passport::parse("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm".to_string());
            assert!(passport::has_required_fields(&passport));
        }

        {
            let passport = passport::parse("pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm".to_string());
            assert!(!passport::has_required_fields(&passport));
        }
    }
}