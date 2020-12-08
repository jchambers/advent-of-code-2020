use std::collections::{HashMap, HashSet};

use regex::Regex;

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

    static ref VALID_EYE_COLORS: HashSet<String> = {
        let mut eye_colors = HashSet::new();
        eye_colors.insert(String::from("amb"));
        eye_colors.insert(String::from("blu"));
        eye_colors.insert(String::from("brn"));
        eye_colors.insert(String::from("gry"));
        eye_colors.insert(String::from("grn"));
        eye_colors.insert(String::from("hzl"));
        eye_colors.insert(String::from("oth"));
        eye_colors
    };
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

pub fn is_valid(passport: &HashMap<String, String>) -> bool {
    if !has_required_fields(passport) {
        return false;
    }

    for (key, value) in passport {
        if !match key.as_str() {
            "byr" => year_valid(value, 1920, 2002),
            "iyr" => year_valid(value, 2010, 2020),
            "eyr" => year_valid(value, 2020, 2030),
            "hgt" => height_valid(value),
            "hcl" => hair_color_valid(value),
            "ecl" => eye_color_valid(value),
            "pid" => passport_id_valid(value),
            _ => true
        } {
            return false;
        }
    }

    true
}

fn year_valid(year: &String, min: u32, max: u32) -> bool {
    if let Ok(year) = year.parse::<u32>() {
        year >= min && year <= max
    } else {
        false
    }
}

fn height_valid(height: &String) -> bool {
    lazy_static! {
        static ref HEIGHT_RE: Regex = Regex::new("([0-9]+)(in|cm)").unwrap();
    }

    if let Some(captures) = HEIGHT_RE.captures(height) {
        let magnitude:u32 = captures[1].parse().unwrap();
        let unit = captures[2].to_ascii_lowercase();

        if unit == "in" {
            magnitude >= 59 && magnitude <= 76
        } else {
            magnitude >= 150 && magnitude <= 193
        }
    } else {
        false
    }
}

fn hair_color_valid(hair_color: &String) -> bool {
    lazy_static! {
        static ref HAIR_COLOR_RE: Regex = Regex::new("^#[0-9a-fA-F]{6}$").unwrap();
    }

    HAIR_COLOR_RE.is_match(hair_color)
}

fn eye_color_valid(eye_color: &String) -> bool {
    VALID_EYE_COLORS.contains(eye_color)
}

fn passport_id_valid(passport_id: &String) -> bool {
    lazy_static! {
        static ref PASSPORT_ID_RE: Regex = Regex::new("^[0-9]{9}$").unwrap();
    }

    PASSPORT_ID_RE.is_match(passport_id)
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

    #[test]
    fn year_valid() {
        assert!(passport::year_valid(&String::from("1920"), 1920, 2002));
        assert!(passport::year_valid(&String::from("2002"), 1920, 2002));
        assert!(!passport::year_valid(&String::from("1900"), 1920, 2002));
        assert!(!passport::year_valid(&String::from("2020"), 1920, 2002));
        assert!(!passport::year_valid(&String::from("every year"), 1920, 2002));
    }

    #[test]
    fn height_valid() {
        assert!(passport::height_valid(&String::from("150cm")));
        assert!(passport::height_valid(&String::from("193cm")));
        assert!(passport::height_valid(&String::from("59in")));
        assert!(passport::height_valid(&String::from("76in")));
        assert!(!passport::height_valid(&String::from("149cm")));
        assert!(!passport::height_valid(&String::from("194cm")));
        assert!(!passport::height_valid(&String::from("58in")));
        assert!(!passport::height_valid(&String::from("77in")));
        assert!(!passport::height_valid(&String::from("tall")));
    }

    #[test]
    fn hair_color_valid() {
        assert!(passport::hair_color_valid(&String::from("#ff00ff")));
        assert!(!passport::hair_color_valid(&String::from("#ff00ffab")));
        assert!(!passport::hair_color_valid(&String::from("#brown")));
        assert!(!passport::hair_color_valid(&String::from("#ff")));
        assert!(!passport::hair_color_valid(&String::from("purple")));
    }

    #[test]
    fn eye_color_valid() {
        assert!(passport::eye_color_valid(&String::from("amb")));
        assert!(passport::eye_color_valid(&String::from("blu")));
        assert!(passport::eye_color_valid(&String::from("brn")));
        assert!(passport::eye_color_valid(&String::from("gry")));
        assert!(passport::eye_color_valid(&String::from("grn")));
        assert!(passport::eye_color_valid(&String::from("hzl")));
        assert!(passport::eye_color_valid(&String::from("oth")));
        assert!(!passport::eye_color_valid(&String::from("yak")));
    }

    #[test]
    fn passport_id_valid() {
        assert!(passport::passport_id_valid(&String::from("012345678")));
        assert!(!passport::passport_id_valid(&String::from("0123456789")));
        assert!(!passport::passport_id_valid(&String::from("01234567")));
        assert!(!passport::passport_id_valid(&String::from("01234567a")));
    }
}