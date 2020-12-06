use std::fs;

use regex::Regex;

pub fn find_solution() {
    let contents = fs::read_to_string("input/day02.txt")
        .expect("Something went wrong reading the file");
    let entries = PasswordEntry::parse_entries(contents.as_str());
    let p1 = entries.clone()
        .into_iter()
        .filter(p1_is_valid)
        .count();
    println!("{}", p1);
    let p2 = entries.clone()
        .into_iter()
        .filter(p2_is_valid)
        .count();
    println!("{}", p2);
}

#[derive(Clone, Debug)]
struct PasswordEntry {
    policy: PasswordPolicy,
    password: String,
}

#[derive(Clone, Debug)]
struct PasswordPolicy {
    indices: (u8, u8),
    ch: char,
}

impl PasswordEntry {
    fn parse_entries(s: &str) -> Vec<PasswordEntry> {
        lazy_static! {
            static ref RE: Regex = Regex::new("(?P<i0>\\d+)-(?P<i1>\\d+) (?P<ch>[a-z]{1}): (?P<password>.*)").unwrap();
        }
        RE.captures_iter(s)
            .map(|it| {
                PasswordEntry {
                    password: String::from(&it["password"]),
                    policy: PasswordPolicy {
                        ch: it["ch"].chars().nth(0).unwrap(),
                        indices: (
                            it["i0"].parse::<u8>().unwrap(),
                            it["i1"].parse::<u8>().unwrap()
                        ),
                    },
                }
            })
            .collect::<Vec<PasswordEntry>>()
    }
}

fn p1_is_valid(entry: &PasswordEntry) -> bool {
    let required_char_count = entry.password.chars()
        .filter(|it| *it == entry.policy.ch)
        .count() as u8;
    entry.policy.indices.0 <= required_char_count && required_char_count <= entry.policy.indices.1
}

fn p2_is_valid(entry: &PasswordEntry) -> bool {
    let chars = entry.password.chars();
    let maybe_ch0 = chars.clone().nth((entry.policy.indices.0 - 1) as usize);
    let maybe_ch1 = chars.clone().nth((entry.policy.indices.1 - 1) as usize);
    let ch0 = maybe_ch0.unwrap_or('\0');
    let ch1 = maybe_ch1.unwrap_or('\0');
    (ch0 == entry.policy.ch && ch1 != entry.policy.ch) || (ch1 == entry.policy.ch && ch0 != entry.policy.ch)
}