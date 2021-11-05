// AOC 2020 day 19

#[derive(Debug, Clone)]
enum Rule {
    Atom(char),
    RuleList(Vec<Vec<usize>>),
}

// recursive function that walks a message returning a vector of the message lengths matched (so far)
fn resolve(rules: &[Rule], msg: &Vec<char>, msg_idx: usize, rule_idx: usize) -> Vec<usize> {
    if msg_idx >= msg.len() {
        Vec::new()
    } else {
        match rules[rule_idx] {
            Rule::Atom(atom_val) => {
                if msg[msg_idx] == atom_val {
                    [msg_idx + 1].to_vec() // matched an atom, return the length matched (so far)
                } else {
                    Vec::new()
                }
            }
            Rule::RuleList(ref rlist) => {
                let mut matched_indices = Vec::new();

                // recursively apply the current rule(s) at the current point (msg_idx) in the message
                for rules_set in rlist {
                    let mut current_msg_indices = [msg_idx].to_vec();

                    for rule_next_idx in rules_set {
                        let mut new_msg_indices = Vec::new();

                        for current_msg_idx in &current_msg_indices {
                            // by appending the result of the recursive call we are doing a BFS
                            new_msg_indices.append(&mut resolve(
                                rules,
                                msg,
                                *current_msg_idx,
                                *rule_next_idx,
                            ));
                        }

                        current_msg_indices = new_msg_indices;
                        if current_msg_indices.is_empty() {
                            break;
                        }
                    }
                    matched_indices.append(&mut current_msg_indices);
                }

                matched_indices
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    const LIMIT: usize = 999;
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut messages: Vec<String> = vec![];
    let mut rules: Vec<Rule> = vec![Rule::Atom('्'); LIMIT];
    let mut collecting_rules = true;
    let mut largest_rule_idx = 0;

    while reader.read_line(&mut inputstr)? != 0 {
        if inputstr.trim() == "" {
            collecting_rules = false;
        } else if collecting_rules {
            let instr = inputstr.trim().to_string();
            let mut split = instr.split(": ");
            let idx: usize = split.next().unwrap().parse().unwrap();
            let rest: String = split.next().unwrap().parse().unwrap();

            let rule: Rule = match rest.as_str() {
                "\"a\"" => Rule::Atom('a'),
                "\"b\"" => Rule::Atom('b'),
                _ => Rule::RuleList(
                    rest.split(" | ")
                        .map(|str| {
                            str.split(' ')
                                .map(|s| s.parse::<usize>().unwrap())
                                .collect()
                        })
                        .collect(),
                ),
            };

            if idx > largest_rule_idx {
                largest_rule_idx = idx;
                if idx > LIMIT - 1 {
                    panic!("rule index too large implementation: {}", idx);
                }
            }
            rules[idx] = rule;
        } else {
            messages.push(inputstr.trim().to_string())
        }

        inputstr.clear();
    }

    // clean up the rules vector
    rules.truncate(largest_rule_idx + 1);

    let mut ans = 0;
    for msg in &messages {
        let indices = resolve(&rules, &msg.chars().collect(), 0, 0);
        ans += indices.iter().any(|i| *i == msg.len()) as usize;
    }
    println!("aoc19a: {}", ans);

    // update rules for part two of the problem
    rules[8] = Rule::RuleList(vec![vec![42], vec![42, 8]]);
    rules[11] = Rule::RuleList(vec![vec![42, 31], vec![42, 11, 31]]);
    ans = 0;
    for msg in &messages {
        let indices = resolve(&rules, &msg.chars().collect(), 0, 0);
        ans += indices.iter().any(|i| *i == msg.len()) as usize;
    }
    println!("aoc19b: {}", ans);

    Ok(())
}
