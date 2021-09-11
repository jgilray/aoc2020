// AOC 2020 day 9

// function that checks the validity of the passed vector returning the index of the first
// failing entry or the length of v if no errors are found
fn check_ivec(preamble_len: usize, v: &[usize]) -> usize {
    'lp: for i in preamble_len..v.len() {
        for j in i - preamble_len..i {
            for k in j + 1..i {
                if v[j] + v[k] == v[i] {
                    continue 'lp;
                }
            }
        }
        return i;
    }

    v.len()
}

// search for a contiguous set of numbers that add up to inv and return the sum of the two largest
// otherwise return 0
fn find_encryption_weakness(inv: usize, v: &[usize]) -> usize {
    'lp: for i in 0..v.len() {
        let mut sum = v[i];
        let mut fvec = vec![v[i]];
        for cand in v.iter().skip(i + 1) {
            sum += cand;
            fvec.push(*cand);
            if sum == inv {
                return fvec.iter().min().unwrap() + fvec.iter().max().unwrap();
            } else if sum > inv {
                continue 'lp;
            }
        }
    }

    0
}

fn main() {
    let input = include_str!("../../xmas_numbers.dat");
    // ivec is a vector of numbers
    let ivec = input
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let inv_num = ivec[check_ivec(25, &ivec)];
    println!("aoc9a: {}", inv_num);
    println!("aoc9b: {}", find_encryption_weakness(inv_num, &ivec));
}
