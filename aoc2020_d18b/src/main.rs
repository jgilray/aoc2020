// AOC 2020 day 18 part 2 using peg crate

extern crate peg;

// My first peg use... I couldn't even figure out how to ignore whitespace
// so I stripped it out
peg::parser!(grammar arithmetic() for str {
    rule number() -> u64
        = n:$(['0'..='9']+) { n.parse().unwrap() }

    pub(crate) rule calculate() -> u64 = precedence!{
        x:(@) "*" y:@ { x * y }
        --
        x:(@) "+" y:@ { x + y }
        --
        "(" v:calculate() ")" { v }
        n:number() {n}
    }
});

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut sum = 0_u64;

    while reader.read_line(&mut inputstr)? != 0 {
        let s: String = inputstr
            .trim()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();
        if arithmetic::calculate(&s).is_ok() {
            sum += arithmetic::calculate(&s).unwrap();
        }

        inputstr.clear();
    }

    println!("aoc18b: {} ", sum);

    Ok(())
}
