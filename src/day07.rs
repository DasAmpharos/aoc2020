use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::Hash;
use std::iter::FromIterator;

use regex::{Captures, Regex};

pub fn find_solution() {
    let contents = fs::read_to_string("input/day07.txt")
        .expect("Something went wrong reading the file");

    lazy_static! {
        static ref RE: Regex = Regex::new("(?P<parent_bag>.*) bags contain (?P<child_bags>.*).").unwrap();
    }

    let bag_policies = RE.captures_iter(contents.as_str())
        .map(|it| BagPolicy::from(it))
        .collect::<Vec<BagPolicy>>();
    let policy_registry = to_hash_map(
        &bag_policies,
        |policy: &BagPolicy| policy.color.clone(),
        |policy: &BagPolicy| to_hash_map(
            &policy.child_bags,
            |entry: &(i32, String)| entry.1.clone(),
            |entry: &(i32, String)| entry.0,
        ),
    );
    println!("{:?}", collect_policies(&policy_registry, "shiny gold").len());
    println!("{:?}", get_required_bags("shiny gold", 1, &policy_registry));
}

#[derive(Clone, Debug)]
struct BagPolicy {
    color: String,
    child_bags: Vec<(i32, String)>,
}

impl BagPolicy {
    fn from(captures: Captures) -> BagPolicy {
        lazy_static! {
            static ref CHILD_BAG_RE: Regex = Regex::new("(?P<quantity>[0-9]|no) (?P<color>.*) bag[s]?").unwrap();
        }
        BagPolicy {
            color: String::from(&captures["parent_bag"]),
            child_bags: (&captures["child_bags"]).split(", ")
                .flat_map(|it| CHILD_BAG_RE.captures_iter(it))
                .map(|it| {
                    let color = normalize_color(&it["color"]);
                    let quantity = normalize_quantity(&it["quantity"]);
                    (quantity, color)
                })
                .collect::<Vec<(i32, String)>>(),
        }
    }
}

#[inline]
fn normalize_color(color: &str) -> String {
    if color != "other" {
        String::from(color)
    } else {
        String::from("")
    }
}

#[inline]
fn normalize_quantity(quantity: &str) -> i32 {
    if quantity != "no" {
        quantity.parse::<i32>().unwrap()
    } else {
        0
    }
}

#[inline]
fn to_hash_map<T, K, V>(it: &Vec<T>, k_fn: fn(&T) -> K, v_fn: fn(&T) -> V) -> HashMap<K, V>
    where K: Hash,
          K: Eq {
    let mut map = HashMap::<K, V>::new();
    for element in it {
        map.insert(k_fn(element), v_fn(element));
    }
    map
}

fn collect_policies(
    policy_registry: &HashMap<String, HashMap<String, i32>>,
    color: &str,
) -> HashSet<String> {
    let mut colors = HashSet::<String>::new();
    let mut visited = HashSet::<String>::new();
    colors.insert(String::from(color));
    loop {
        let policies = colors.iter()
            .flat_map(|it| collect_policies_aux(policy_registry, it))
            .filter(|it| it != color)
            .collect::<HashSet<String>>();
        policies.iter().for_each(|it| { visited.insert(it.clone()); });
        if policies.len() == 0 { break; }
        colors = policies;
    }
    visited
}

fn collect_policies_aux(
    policy_registry: &HashMap<String, HashMap<String, i32>>,
    color: &str,
) -> HashSet<String> {
    policy_registry.into_iter()
        .filter(|it| it.1.contains_key(color))
        .map(|it| it.0.clone())
        .collect::<HashSet<String>>()
}

fn get_required_bags(
    color: &str,
    quantity: i32,
    policy_registry: &HashMap<String, HashMap<String, i32>>,
) -> i32 {
    let policy = &policy_registry[&String::from(color)];
    let mut sum = (quantity * policy.values().sum::<i32>());
    if sum != 0 {
        sum += policy.iter()
            .map(|it| get_required_bags(it.0.as_str(), *it.1 * quantity, policy_registry))
            .sum::<i32>()
    }
    sum
}
