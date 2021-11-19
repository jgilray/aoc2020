// AOC 2020 day 25

fn main() -> std::io::Result<()> {
    const MOD: usize = 20_201_227;
    // To do the example, let pk1 = 5_764_801; let pk2 = 17_807_724;
    let pk1 = 15_628_416;
    let pk2 = 11_161_639;

    let mut value = 1;
    let mut subject = 7;
    let mut loop_size = 0;

    for ls in 1.. {
        value = (value * subject) % MOD;
        if value == pk1 || value == pk2 {
            loop_size = ls;
            break;
        }
    }

    subject = if value == pk1 { pk2 } else { pk1 };
    value = 1;
    for _ in 0..loop_size {
        value = (value * subject) % MOD;
    }

    println!("aoc25a: {}", value);

    Ok(())
}
