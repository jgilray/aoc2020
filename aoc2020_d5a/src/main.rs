// AOC 2020 day 5

fn convert_seatnum(s: &str) -> usize {
    let mut row_min = 0;
    let mut row_max = 127;
    let mut col_min = 0;
    let mut col_max = 7;

    for c in s.chars() {
        match c {
            'B' => row_min = (row_max + row_min) / 2 + 1,
            'F' => row_max = (row_max + row_min) / 2,
            'R' => col_min = (col_max + col_min) / 2 + 1,
            'L' => col_max = (col_max + col_min) / 2,
            _ => panic!("bad boarding pass: {}", s),
        }
    }
    row_min * 8 + col_min
}

fn main() {
    let input = include_str!("../../boarding_passes.dat");
    let mut seatvec = input
        .lines()
        .map(|line| convert_seatnum(line))
        .collect::<Vec<_>>();

    let mut l_id = 0;
    let _ = seatvec
        .iter()
        .map(|sid| {
            if *sid > l_id {
                l_id = *sid
            }
            l_id
        })
        .last();
    println!("aoc5a: {}", l_id);

    seatvec.sort_unstable();
    let first_seat_id = seatvec[0];
    for (i, s) in seatvec.iter().enumerate() {
        if i + first_seat_id != *s {
            println!("aoc5b: {}", i + first_seat_id);
            break;
        }
    }
}
