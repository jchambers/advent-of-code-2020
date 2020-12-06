use std::error;

#[derive(Debug, PartialEq)]
pub struct PasswordPolicy {
    character: char,
    min: u32,
    max: u32,
}

pub fn parse(password_db_line:&str) -> Result<(PasswordPolicy, &str), Box<dyn error::Error>> {
    let hyphen = password_db_line.find('-')
        .ok_or(simple_error::simple_error!("Could not find hyphen in password database line"))?;

    let first_space = password_db_line.find(' ')
        .ok_or(simple_error::simple_error!("Could not find space in password database line"))?;

    let password_separator = password_db_line.find(": ")
        .ok_or(simple_error::simple_error!("Could not find password separator in password database line"))?;

    let chars: Vec<char> = password_db_line.chars().collect();

    let min: u32 = password_db_line[0..hyphen].parse()?;
    let max: u32 = password_db_line[(hyphen + 1)..first_space].parse()?;
    let character = chars[first_space + 1];
    let password = &password_db_line[(password_separator + 2)..];

    Ok((PasswordPolicy {
        character,
        min,
        max
    }, password))
}

pub fn is_valid(policy: PasswordPolicy, password: &str) -> bool {
    let mut char_count = 0;

    for c in password.chars() {
        if c == policy.character {
            char_count += 1;
        }
    }

    char_count >= policy.min && char_count <= policy.max
}

#[cfg(test)]
mod tests {
    use crate::password;
    use crate::password::PasswordPolicy;

    #[test]
    fn parse() {
        let (policy, password) = password::parse("1-3 a: abcde").unwrap();

        assert_eq!(PasswordPolicy {
            character: 'a',
            min: 1,
            max: 3
        }, policy);

        assert_eq!("abcde", password);
    }

    #[test]
    fn is_valid() {
        {
            let policy = PasswordPolicy {
                character: 'a',
                min: 1,
                max: 3
            };

            assert_eq!(true, password::is_valid(policy, "abcde"));
        }

        {
            let policy = PasswordPolicy {
                character: 'b',
                min: 1,
                max: 3
            };

            assert_eq!(false, password::is_valid(policy, "cdefg"));
        }
    }
}
