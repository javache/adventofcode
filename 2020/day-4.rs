use std::collections::HashMap;
use std::io::{self, BufRead};

const REQUIRED_FIELDS: &'static [&'static str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const VALID_EYE_COLORS: &'static [&'static str] =
    &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn in_range(input: &str, min: i32, max: i32) -> bool {
    match input.parse::<i32>() {
        Ok(n) => (min..=max).contains(&n),
        _ => false,
    }
}

fn check_passport(passport: &HashMap<String, String>) -> bool {
    if !REQUIRED_FIELDS
        .iter()
        .all(|f| passport.contains_key::<str>(f))
    {
        return false;
    }

    passport.iter().all(|(key, value)| match key.as_str() {
        "byr" => in_range(value, 1920, 2002),
        "iyr" => in_range(value, 2010, 2020),
        "eyr" => in_range(value, 2020, 2030),
        "hgt" => match (value.ends_with("cm"), value.ends_with("in")) {
            (true, _) => in_range(&value[..value.len() - 2], 150, 193),
            (_, true) => in_range(&value[..value.len() - 2], 59, 76),
            _ => false,
        },
        "hcl" => {
            value.len() == 7 && &value[0..1] == "#" && value[1..6].chars().all(|c| c.is_digit(16))
        }
        "ecl" => VALID_EYE_COLORS.iter().any(|&i| i == value),
        "pid" => value.len() == 9 && value.chars().all(char::is_numeric),
        _ => true,
    })
}

fn main() -> io::Result<()> {
    let mut fields: HashMap<String, String> = HashMap::new();
    let mut valid = 0;

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            // Complete entry, check
            if check_passport(&fields) {
                valid += 1;
            }
            fields.clear();
        } else {
            let entries = line.split(' ');
            for entry in entries {
                let v: Vec<&str> = entry.split(':').collect();
                if let [key, value] = v.as_slice() {
                    fields.insert(key.to_string(), value.to_string());
                }
            }
        }
    }
    if check_passport(&fields) {
        valid += 1;
    }

    println!("{} valid passports", valid);

    Ok(())
}
