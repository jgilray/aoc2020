// AOC 2020 day 1

fn main() {
    let input = include_str!("../../expenses.dat");
    let values = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let result = generate_pairs(values.as_slice())
        .into_iter()
        .filter(|x| x.0 + x.1 == 2020)
        .map(|x| x.0 * x.1)
        .collect::<Vec<_>>();
    println!("aoc1a: {:?}", result);

    let result = generate_triples(values.as_slice())
        .iter()
        .filter(|x| x.0 + x.1 + x.2 == 2020)
        .map(|x| x.0 * x.1 * x.2)
        .collect::<Vec<_>>();
    println!("aoc1b: {:?}", result);
}

fn generate_pairs(xs: &[i32]) -> Vec<(i32, i32)> {
    let mut pairs = Vec::<(i32, i32)>::new();

    for (xi, x) in xs.iter().enumerate() {
        let ys = &xs[xi + 1..];
        for y in ys.iter() {
            pairs.push((*x, *y));
        }
    }

    pairs
}

fn generate_triples(xs: &[i32]) -> Vec<(i32, i32, i32)> {
    let mut ps = Vec::<(i32, i32, i32)>::new();

    for (xi, x) in xs.iter().enumerate() {
        let ys = &xs[xi + 1..];
        for (yi, y) in ys.iter().enumerate() {
            let zs = &ys[yi + 1..];
            for z in zs.iter() {
                ps.push((*x, *y, *z));
            }
        }
    }

    ps
}

#[test]
fn test_generate_triples() {
    let values: Vec<i32> = vec![0, 1, 2, 3];
    let groups = generate_triples(values.as_slice());

    let expected: Vec<_> = vec![(0, 1, 2), (0, 1, 3), (0, 2, 3), (1, 2, 3)];
    assert_eq!(groups, expected);
}

#[test]
fn test_generate_pairs() {
    let values: Vec<i32> = vec![0, 1, 2];
    let pairs = generate_pairs(values.as_slice());

    let expected: Vec<(i32, i32)> = vec![(0, 1), (0, 2), (1, 2)];
    assert_eq!(pairs, expected);
}
