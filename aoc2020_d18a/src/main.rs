// AOC 2020 day 18

#[derive(PartialEq, Debug)]
enum Operator {
    Addition,
    Multiplication,
    None,
}

// simple left-to-right arithmatic expression evaluator
#[derive(Debug)]
struct L2Rarith {
    num: u64,
}

impl L2Rarith {
    fn new(s: &str) -> Self {
        Self {
            num: L2Rarith::resolve(&mut s.chars()),
        }
    }

    fn resolve(citer: &mut std::str::Chars) -> u64 {
        let mut retval: u64 = 0;
        let mut op = Operator::None;
        let mut val: u64 = 0;
        while !citer.as_str().is_empty() {
            let c = citer.next().expect("expression ended unexpectedly");
            match c {
                '0'..='9' => {
                    val *= 10;
                    val += (c as u8 - b'0') as u64;
                }
                ' ' => {
                    if val != 0 {
                        if op != Operator::None {
                            retval = L2Rarith::mathop(retval, op, val);
                        } else {
                            retval = val;
                        }
                        val = 0;
                        op = Operator::None;
                    }
                }
                '*' => op = Operator::Multiplication,
                '+' => op = Operator::Addition,
                '(' => {
                    val = L2Rarith::resolve(citer);
                    if op != Operator::None {
                        retval = L2Rarith::mathop(retval, op, val);
                    } else {
                        retval = val;
                    }
                    val = 0;
                    op = Operator::None;
                }
                ')' => {
                    if op != Operator::None {
                        retval = L2Rarith::mathop(retval, op, val);
                    }
                    return retval;
                }
                _ => panic!("illegal character in expression: {}", c),
            }
        }
        if op != Operator::None {
            retval = L2Rarith::mathop(retval, op, val);
        }
        retval
    }

    fn mathop(lhs: u64, op: Operator, rhs: u64) -> u64 {
        match op {
            Operator::Addition => lhs + rhs,
            Operator::Multiplication => lhs * rhs,
            Operator::None => panic!("Unexpected null operator"),
        }
    }
}

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut sum = 0;

    while reader.read_line(&mut inputstr)? != 0 {
        let l2r = L2Rarith::new(inputstr.trim());
        sum += l2r.num;

        inputstr.clear();
    }

    println!("aoc18a: {} ", sum);

    Ok(())
}
