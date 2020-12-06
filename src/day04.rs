use std::fs;
use std::ops::RangeInclusive;

use regex::Regex;

pub fn find_solution() {
    let contents = fs::read_to_string("input/day04.txt")
        .expect("Something went wrong reading the file");

    lazy_static! {
        static ref PASSPORT_RE: Regex = Regex::new("\n{2}").unwrap();
    }

    let entries = PASSPORT_RE.split(contents.as_str())
        .collect::<Vec<&str>>()
        .into_iter()
        .map(PassportEntry::parse)
        .collect::<Vec<PassportEntry>>();

    let p1_valid = entries.clone()
        .into_iter()
        .filter(p1_is_valid)
        .count();
    println!("{}", p1_valid);
    let p2_valid = entries.clone()
        .into_iter()
        .filter(p2_is_valid)
        .count();
    println!("{}", p2_valid);
}

#[derive(Clone, Debug)]
struct PassportEntry {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl PassportEntry {
    fn parse(s: &str) -> PassportEntry {
        lazy_static! {
            static ref FIELD_RE: Regex = Regex::new("(?P<field_name>.*):(?P<field_value>.*)").unwrap();
        }

        let mut byr: Option<String> = None;
        let mut iyr: Option<String> = None;
        let mut eyr: Option<String> = None;
        let mut hgt: Option<String> = None;
        let mut hcl: Option<String> = None;
        let mut ecl: Option<String> = None;
        let mut pid: Option<String> = None;
        let mut cid: Option<String> = None;

        let fields = s.split_whitespace();
        for field in fields {
            FIELD_RE.captures_iter(field)
                .for_each(|it| {
                    let field_name = &it["field_name"];
                    let field_value = Some(String::from(&it["field_value"]));
                    match field_name {
                        "byr" => byr = field_value,
                        "iyr" => iyr = field_value,
                        "eyr" => eyr = field_value,
                        "hgt" => hgt = field_value,
                        "hcl" => hcl = field_value,
                        "ecl" => ecl = field_value,
                        "pid" => pid = field_value,
                        "cid" => cid = field_value,
                        _ => eprintln!("Unrecognized field_name: {}", field_name)
                    }
                });
        }

        PassportEntry {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            ecl,
            pid,
            cid,
        }
    }
}

fn p1_is_valid(entry: &PassportEntry) -> bool {
    entry.byr.is_some() &&
        entry.iyr.is_some() &&
        entry.eyr.is_some() &&
        entry.hgt.is_some() &&
        entry.hcl.is_some() &&
        entry.ecl.is_some() &&
        entry.pid.is_some()
}

fn p2_is_valid(entry: &PassportEntry) -> bool {
    lazy_static! {
        static ref HCL_RE: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
        static ref ECL_VEC: Vec<&'static str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        static ref PID_RE: Regex = Regex::new("^[0-9]{9}$").unwrap();
    }

    if (&entry.byr).as_ref()
        .filter(|it| is_year_valid(it, 1920, 2002))
        .is_none() {
        return false;
    }

    if (&entry.iyr).as_ref()
        .filter(|it| is_year_valid(it, 2010, 2020))
        .is_none() {
        return false;
    }

    if (&entry.eyr).as_ref()
        .filter(|it| is_year_valid(it, 2020, 2030))
        .is_none() {
        return false;
    }

    if (&entry.hgt).as_ref()
        .filter(|it| is_hgt_valid(it))
        .is_none() {
        return false;
    }

    if (&entry.hcl).as_ref()
        .filter(|it| HCL_RE.is_match(it.as_str()))
        .is_none() {
        return false;
    }

    if (&entry.ecl).as_ref()
        .filter(|it| ECL_VEC.contains(&it.as_str()))
        .is_none() {
        return false;
    }

    let matches = (&entry.pid).as_ref()
        .filter(|it| PID_RE.is_match(it.as_str()))
        .is_some();
    println!("{:?} - is_valid: {}", entry.pid, matches);
    matches
}

fn is_year_valid(year: &String, min: i32, max: i32) -> bool {
    year.parse::<i32>()
        .map(|it| min <= it && it <= max)
        .unwrap_or(false)
}

fn is_hgt_valid(hgt: &String) -> bool {
    lazy_static! {
        static ref HGT_RE: Regex = Regex::new("(?P<value>[0-9]{2,3})(?P<unit>cm|in){1}").unwrap();
        static ref HGT_CM: RangeInclusive<i32> = 150..=193;
        static ref HGT_IN: RangeInclusive<i32> = 59..=76;
    }

    if let Some(capture) = HGT_RE.captures_iter(hgt.as_str()).nth(0) {
        let unit: &str = &capture["unit"];
        if let Ok(value) = (&capture["value"]).parse::<i32>() {
            return if unit == "cm" {
                150 <= value && value <= 193
            } else {
                59 <= value && value <= 76
            };
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_hgt_valid() {
        assert_eq!(is_hgt_valid(&String::from("1")), false);
        assert_eq!(is_hgt_valid(&String::from("12")), false);
        assert_eq!(is_hgt_valid(&String::from("123")), false);
        assert_eq!(is_hgt_valid(&String::from("1234")), false);

        assert_eq!(is_hgt_valid(&String::from("149cm")), false);
        assert_eq!(is_hgt_valid(&String::from("194cm")), false);
        assert_eq!(is_hgt_valid(&String::from("58in")), false);
        assert_eq!(is_hgt_valid(&String::from("78in")), false);

        assert_eq!(is_hgt_valid(&String::from("150cm")), true);
        assert_eq!(is_hgt_valid(&String::from("193cm")), true);
        assert_eq!(is_hgt_valid(&String::from("59in")), true);
        assert_eq!(is_hgt_valid(&String::from("76in")), true);
    }
}
