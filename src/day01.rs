use std::fs;

pub fn find_solution() {
    let contents = fs::read_to_string("input/day01.txt")
        .expect("Something went wrong reading the file");
    let lines = contents.lines()
        .map(|it| it.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let p1 = find_part1(&lines).unwrap();
    println!("part1: {}", lines[p1.0] * lines[p1.1]);
    let p2 = find_part2(&lines).unwrap();
    println!("part2: {}", lines[p2.0] * lines[p2.1] * lines[p2.2]);

}

fn find_part1(lines: &Vec<i32>) -> Option<(usize, usize)> {
    let mut x = 0;
    while x < lines.len() {
        let mut y = x + 1;
        while y < lines.len() {
            if lines[x] + lines[y] == 2020 {
                return Some((x, y));
            }
            y += 1;
        }
        x += 1;
    }
    return None;
}

fn find_part2(lines: &Vec<i32>) -> Option<(usize, usize, usize)> {
    let mut x = 0;
    while x < lines.len() {
        let mut y = x + 1;
        while y < lines.len() {
            let mut z = y + 1;
            while z < lines.len() {
                if lines[x] + lines[y] + lines[z] == 2020 {
                    return Some((x, y, z));
                }
                z += 1;
            }
            y += 1;
        }
        x += 1;
    }
    return None;
}
