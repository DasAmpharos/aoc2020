use std::fs;

use substring::Substring;

pub fn find_solution() {
    let contents = fs::read_to_string("input/day05.txt")
        .expect("Something went wrong reading the file");

    let ids = contents.lines()
        .map(calculate_seat_id)
        .collect::<Vec<i32>>();
    println!("{}", find_solution_p1(&ids));
    println!("{}", find_solution_p2(&ids));
}

fn find_solution_p1(ids: &Vec<i32>) -> i32 {
    ids.into_iter()
        .fold(std::i32::MIN, |a, b| a.max(*b))
}

fn find_solution_p2(ids: &Vec<i32>) -> i32 {
    let mut sorted = ids.clone();
    sorted.sort_by(|a, b| a.cmp(b));

    let mut idx = 0;
    loop {
        if idx + 1 >= sorted.len() { break; }

        let current = sorted[idx];
        let next = sorted[idx + 1];
        if next - current == 2 {
            println!("{}-{}", current, next);
        }

        idx += 1;
    }
    0
}

fn calculate_seat_id(id: &str) -> i32 {
    let row = calculate_row(id.substring(0, 7), (0, 127));
    let col = calculate_col(id.substring(7, id.len()), (0, 7));
    row * 8 + col
}

fn calculate_row(id: &str, initial_range: (i32, i32)) -> i32 {
    let mut range = initial_range;

    let mut chars = id.chars();
    while let Some(ch) = chars.next() {
        if ch == 'F' {
            range = (range.0, ((range.1 - range.0) / 2) + range.0)
        } else {
            range = (((range.1 - range.0) / 2) + range.0 + 1, range.1)
        }
    }

    if range.0 != range.1 { panic!("Unable to find row for {}", id); }
    range.0
}

fn calculate_col(id: &str, initial_range: (i32, i32)) -> i32 {
    let mut range = initial_range;

    let mut chars = id.chars();
    while let Some(ch) = chars.next() {
        if ch == 'L' {
            range = (range.0, ((range.1 - range.0) / 2) + range.0)
        } else {
            range = (((range.1 - range.0) / 2) + range.0 + 1, range.1)
        }
    }
    if range.0 != range.1 { panic!("Unable to find col for {}", id); }
    range.0
}
