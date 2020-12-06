use std::collections::HashSet;
use std::fs;

use regex::Regex;

pub fn find_solution() {
    let contents = fs::read_to_string("input/day06.txt")
        .expect("Something went wrong reading the file");

    lazy_static! {
        static ref GROUP_RE: Regex = Regex::new("\n{2}").unwrap();
    }

    let groups = GROUP_RE.split(contents.as_str())
        .collect::<Vec<&str>>();
    println!("{}", p1_find_solution(&groups));
    println!("{}", p2_find_solution(&groups));
}

fn p1_find_solution(groups: &Vec<&str>) -> usize {
    groups.iter()
        .map(|it| union_answers(it))
        .fold(0, |acc, it| acc + it.len())
}

fn p2_find_solution(groups: &Vec<&str>) -> usize {
    groups.iter()
        .map(|it| intersection_answers(it))
        .filter(|it| it.is_some())
        .map(|it| it.unwrap())
        .fold(0, |acc, it| acc + it.len())
}

fn union_answers(group: &str) -> HashSet<char> {
    group.split_whitespace()
        .flat_map(|it| it.chars())
        .collect::<HashSet<char>>()
}

fn intersection_answers(group: &str) -> Option<HashSet<char>> {
    let sets = group.split_whitespace()
        .map(|it| it.chars().collect::<HashSet<char>>())
        .collect::<Vec<HashSet<char>>>();

    sets.iter().next()
        .map(|initial_set| sets.iter()
            .fold(initial_set.iter().cloned().collect::<HashSet<char>>(), |acc, it| intersect(acc, it)))
}

#[inline]
fn intersect(lhs: HashSet<char>, rhs: &HashSet<char>) -> HashSet<char> {
    lhs.intersection(rhs)
        .into_iter().cloned()
        .collect::<HashSet<char>>()
}
