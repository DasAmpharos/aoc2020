use std::fs;
use std::str::Lines;

pub fn find_solution() {
    let contents = fs::read_to_string("input/day03.txt")
        .expect("Something went wrong reading the file");

    let lines = contents.lines();
    let rows = lines.clone()
        .count();
    let cols = lines.clone()
        .nth(0).unwrap()
        .len();
    let matrix = parse_matrix(contents.lines());

    let p0 = check_trajectory((1, 1), &matrix, rows, cols) as i64;
    let p1 = check_trajectory((1, 3), &matrix, rows, cols) as i64;
    let p2 = check_trajectory((1, 5), &matrix, rows, cols) as i64;
    let p3 = check_trajectory((1, 7), &matrix, rows, cols) as i64;
    let p4 = check_trajectory((2, 1), &matrix, rows, cols) as i64;
    println!("{}", p1);
    println!("{}", p0 * p1 * p2 * p3 * p4);
}

fn check_trajectory(transform: (usize, usize), matrix: &Vec<Vec<usize>>, rows: usize, cols: usize) -> i32 {
    let mut pos: (usize, usize) = (0, 0);
    let mut trees_encountered = 0;
    while (pos.0 + 1) < rows {
        pos.1 = translate_col(pos.1 + transform.1, cols);
        pos.0 += transform.0;
        if has_tree(&pos, &matrix) {
            trees_encountered += 1;
        }
    }
    trees_encountered
}

fn parse_matrix(lines: Lines) -> Vec<Vec<usize>> {
    lines.into_iter()
        .map(|line| {
            (0..line.len()).into_iter()
                .filter(|idx| line
                    .chars()
                    .nth(*idx)
                    .unwrap() == '#'
                )
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

fn has_tree(pos: &(usize, usize), matrix: &Vec<Vec<usize>>) -> bool {
    let row = &matrix[pos.0];
    row.contains(&(pos.1 as usize))
}

fn translate_col(col: usize, cols: usize) -> usize {
    col - ((col / cols) * cols)
}
