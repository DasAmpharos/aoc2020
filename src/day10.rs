use std::fs;

pub fn find_solution() {
    let contents = fs::read_to_string("input/day10.txt")
        .expect("Something went wrong reading the file");

    let mut adapters = contents.lines()
        .map(|it| it.parse::<u64>()
            .expect(format!("Invalid u64: {}", it).as_str()))
        .collect::<Vec<u64>>();
    adapters.sort_by(|lhs, rhs| lhs.cmp(rhs));

    println!("{}", p1_find_solution(&adapters, 0));
}

fn p1_find_solution(adapters: &Vec<u64>, initial_rating: u64) -> u64 {
    let mut differences: (u64, u64) = (0, 0);
    let builtin_rating = tail(&adapters) + 3;
    do_difference(&mut differences, head(&adapters) - initial_rating);
    do_difference(&mut differences, builtin_rating - tail(&adapters));
    for i in 0..adapters.len() {
        if i + 1 >= adapters.len() { break; }
        do_difference(&mut differences, adapters[i + 1] - adapters[i]);
    }
    differences.0 * differences.1
}

fn p2_find_solution(adapters: &Vec<u64>) -> i32 {
    let mut count = 0;
    let builtin_rating = tail(&adapters) + 3;
    for i in 0..adapters.len() {
        let it = adapters.iter().cloned().collect::<Vec<u64>>();
        if is_valid(&it, 0, builtin_rating) { count += 1; }
    }
    count
}

fn p2_find_solution_aux(adapters: &mut Vec<u64>, idx: usize) -> i32 {
    let mut count = 0;
}

fn is_valid(adapters: &Vec<u64>, initial_rating: u64, builtin_rating: u64) -> bool {
    if adapters.len() == 0 { return false; }
    let initial_diff = head(&adapters) - initial_rating;
    let builtin_diff = builtin_rating - tail(&adapters);
    if (initial_diff != 1 && initial_diff != 3) || builtin_diff != 3 { return false; }

    for i in 0..adapters.len() {
        if i + 1 >= adapters.len() { break; }
        let diff = adapters[i + 1] - adapters[i];
        if diff != 0 && diff != 3 { return false; }
    }
    true
}

#[inline]
fn do_difference(differences: &mut (u64, u64), difference: u64) {
    match difference {
        1 => differences.0 += 1,
        3 => differences.1 += 1,
        _ => panic!("Invalid difference {}", difference)
    }
}

#[inline]
fn head<T>(vec: &Vec<T>) -> &T {
    if vec.len() == 0 { panic!("Cannot get head for vec of size 0"); }
    &vec[0]
}

#[inline]
fn tail<T>(vec: &Vec<T>) -> &T {
    if vec.len() == 0 { panic!("Cannot get tail for vec of size 0"); }
    if vec.len() == 1 { return &vec[0]; }
    &vec[vec.len() - 1]
}