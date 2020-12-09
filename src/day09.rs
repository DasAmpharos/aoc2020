use std::fs;
use std::ops::Range;

pub fn find_solution() {
    let contents = fs::read_to_string("input/day09.txt")
        .expect("Something went wrong reading the file");

    let packet = contents.lines()
        .map(|it| it.parse::<u64>()
            .expect(format!("Invalid number: {}", it).as_str()))
        .collect::<Vec<u64>>();
    let p1_solution = p1_find_solution(&packet, 25);
    let p2_solution = p2_find_solution(&packet, p1_solution);
    println!("{}", p1_solution);
    println!("{}", p2_solution);
}

fn p1_find_solution(packet: &Vec<u64>, preamble_len: usize) -> u64 {
    let mut i: usize = 0;
    while (i + preamble_len) < packet.len() {
        let preamble = &packet[i..(i + preamble_len)];
        let maybe_match = find_match(preamble, packet[i + preamble_len]);
        if maybe_match.is_none() { break; }
        i += 1;
    }
    packet[i + preamble_len]
}


fn p2_find_solution(packet: &Vec<u64>, invalid_number: u64) -> u64 {
    let filtered = packet.into_iter()
        .cloned()
        .filter(|it| *it < invalid_number)
        .collect::<Vec<u64>>();

    for start in 0..filtered.len() {
        let mut end = start + 1;
        if end >= filtered.len() { break;}
        loop {
            let sum: u64 = filtered[start..end].iter().sum();
            if sum > invalid_number { break; }
            if sum == invalid_number {
                let mut subset = filtered[start..end].to_vec();
                subset.sort_by(|lhs, rhs| lhs.cmp(rhs));
                return subset[0] + subset[subset.len() - 1];
            }
            end += 1;
        }
    }
    panic!("no solution found :(");
}

fn find_match(preamble: &[u64], n: u64) -> Option<(u64, u64)> {
    let mut rval = None;
    for x in 0..preamble.len() {
        for y in x..preamble.len() {
            if preamble[x] + preamble[y] == n {
                rval = Some((preamble[x], preamble[y]));
                break;
            }
        }
    }
    rval
}
