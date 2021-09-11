// AOC 2020 day 8

use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Instr {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

fn parse_instruction(s: &str) -> Instr {
    let v = s.split(' ').collect::<Vec<_>>();
    let o: isize = v[1].parse().unwrap();
    match v[0] {
        "nop" => Instr::Nop(o),
        "acc" => Instr::Acc(o),
        "jmp" => Instr::Jmp(o),
        _ => panic!("bad instruction code"),
    }
}

// runs the passed program, returning the values of the instruction pointer and the accumulator
// when either the program terminates or an infinite loop is detected
fn run_program(vp: &[Instr]) -> (usize, isize) {
    // the instructions already run
    let mut used: HashSet<usize> = HashSet::new();

    // the accumulator value
    let mut acc: isize = 0;

    // the instruction pointer
    let mut ip: usize = 0;

    // simulate the program
    loop {
        if ip == vp.len() || !used.insert(ip) {
            return (ip, acc);
        } else {
            match vp[ip] {
                Instr::Nop(_) => ip += 1,
                Instr::Acc(val) => {
                    acc += val;
                    ip += 1;
                }
                Instr::Jmp(offset) => ip = (ip as isize + offset) as usize,
            }
        }
    }
}

fn main() {
    let input = include_str!("../../loop_program.dat");
    // ivec is a vector of Instr: the program to run
    let ivec = input
        .lines()
        .map(|line| parse_instruction(line))
        .collect::<Vec<_>>();

    let (_, val) = run_program(&ivec);
    println!("aoc8a: {}", val);

    // build a vector of vectors of the program variants
    let mut vars: Vec<Vec<Instr>> = vec![];
    for (i, inst) in ivec.iter().enumerate() {
        match inst {
            Instr::Jmp(v) => {
                let mut nv = ivec.clone();
                nv[i] = Instr::Nop(*v);
                vars.push(nv);
            }
            Instr::Nop(v) => {
                let mut nv = ivec.clone();
                nv[i] = Instr::Jmp(*v);
                vars.push(nv);
            }
            _ => {}
        }
    }

    // run the variant programs until one terminates
    for vp in &vars {
        let (ip, acc) = run_program(vp);
        if ip == ivec.len() {
            println!("aoc8b: {}", acc);
            break;
        }
    }
}
