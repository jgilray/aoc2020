// AOC 2020 day 13
// i64s would do the job, but I wanted to play with the i128 type

fn closest_shuttle_time(sbv: &Vec<i128>, arrival: i128) -> i128 {
    let mut closest_time = i128::MAX;
    let mut closest_shuttle = 0;
    for s in sbv {
        let div = arrival / (*s);
        let time = (div + 1) * (*s) - arrival;
        if time < closest_time {
            closest_time = time;
            closest_shuttle = *s;
        }
    }

    closest_time * closest_shuttle
}

// Chinese remainder thereom code below from Rosetta Code
fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i128, n: i128) -> Option<i128> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i128], modulii: &[i128]) -> Option<i128> {
    let prod = modulii.iter().product::<i128>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut first_line = true;
    let mut arrival: i128 = 0;
    let mut busses = vec![];
    let mut modulii = vec![];
    let mut residues = vec![];
    let mut r = 0;

    while reader.read_line(&mut inputstr)? != 0 {
        if first_line {
            arrival = inputstr.trim().parse().expect("bad arrival time");
            first_line = false;
        } else {
            let busstr: Vec<&str> = inputstr.trim().split(',').collect();
            for s in busstr {
                if s != "x" {
                    let busnum = s.parse::<i128>().expect("bad bus string");
                    busses.push(busnum);
                    modulii.push(busnum);

                    // deal with residue r = 0 or r > modulus
                    residues.push((busnum - r % busnum) % busnum);
                }
                r += 1;
            }
        }
        inputstr.clear();
    }

    println!("aoc13a: {}", closest_shuttle_time(&busses, arrival));

    match chinese_remainder(&residues, &modulii) {
        Some(ans) => println!("aoc13b: {}", ans),
        None => println!("error: modulii not pairwise coprime"),
    }

    Ok(())
}
