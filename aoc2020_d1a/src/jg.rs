// AOC 2020 day 1

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut v: Vec<usize> = vec![];

    while reader.read_line(&mut inputstr)? != 0 {
        let input = inputstr.trim_end();
        v.push(input.parse::<usize>().unwrap());

        inputstr.clear();
    }

    let mut two_terms = 0;
    let mut three_terms = 0;
    for i in 0..v.len() {
        for j in i..v.len() {
            if v[i] + v[j] == 2020 {
                two_terms = v[i] * v[j];
            }
            for k in j..v.len() {
                if v[i] + v[j] + v[k] == 2020 {
                    three_terms = v[i] * v[j] * v[k];
                }
            }
        }
    }

    println!("aoc1a: {}, aocab: {}", two_terms, three_terms);
    Ok(())
}
