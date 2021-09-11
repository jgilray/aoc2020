// AOC 2020 day 4

#[derive(Debug)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
}

impl Passport {
    fn new(s: &str) -> Passport {
        let mut pw = Passport {
            byr: "".to_string(),
            iyr: "".to_string(),
            eyr: "".to_string(),
            hgt: "".to_string(),
            hcl: "".to_string(),
            ecl: "".to_string(),
            pid: "".to_string(),
            cid: "".to_string(),
        };

        for entry in s.split_whitespace() {
            let ev = entry.split(':').collect::<Vec<_>>();
            match ev[0] {
                "byr" => pw.byr = ev[1].to_string(),
                "iyr" => pw.iyr = ev[1].to_string(),
                "eyr" => pw.eyr = ev[1].to_string(),
                "hgt" => pw.hgt = ev[1].to_string(),
                "hcl" => pw.hcl = ev[1].to_string(),
                "ecl" => pw.ecl = ev[1].to_string(),
                "pid" => pw.pid = ev[1].to_string(),
                "cid" => pw.cid = ev[1].to_string(),
                _ => panic!("bad entry field: {}", ev[0]),
            }
        }
        pw
    }

    fn is_valid_parta(&self) -> bool {
        !(&self.byr == ""
            || &self.iyr == ""
            || &self.eyr == ""
            || &self.hgt == ""
            || &self.hcl == ""
            || &self.ecl == ""
            || &self.pid == "")
    }

    fn is_valid_partb(&self) -> bool {
        // check birth year
        let mut check: bool = match self.byr.parse::<u32>() {
            Err(_) => false,
            Ok(n) => n >= 1920 && n <= 2002,
        };

        // check issue year
        check = check
            && match self.iyr.parse::<u32>() {
                Err(_) => false,
                Ok(n) => n >= 2010 && n <= 2020,
            };

        // check expiration year
        check = check
            && match self.eyr.parse::<u32>() {
                Err(_) => false,
                Ok(n) => n >= 2020 && n <= 2030,
            };

        // check height
        if self.hgt.len() < 4 {
            return false;
        } else {
            // n is the starting byte offset of the second to last character
            let n = self
                .hgt
                .char_indices()
                .rev()
                .nth(1)
                .map(|(a, _)| a)
                .expect("hgt too short");
            let unit = &self.hgt[n..];
            let value = &self.hgt[..n];

            check = check
                && match value.parse::<u32>() {
                    Err(_) => false,
                    Ok(val) => match unit {
                        "cm" => val >= 150 && val <= 193,
                        "in" => val >= 59 && val <= 76,
                        _ => false,
                    },
                }
        }

        // check hair color
        let mut char_num: usize = 0;
        for c in self.hcl.chars() {
            if char_num == 0 {
                if c != '#' {
                    return false;
                }
            } else {
                match c {
                    '0'..='9' | 'a'..='f' => (),
                    _ => return false,
                }
            }
            char_num += 1;
        }
        check = check && char_num == 7;

        // check eye color
        check = check
            && matches!(&self.ecl[..], "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth");

        // check passport id
        char_num = 0;
        for c in self.pid.chars() {
            match c {
                '0'..='9' => (),
                _ => return false,
            }
            char_num += 1;
        }
        check && char_num == 9
    }
}

fn main() {
    let input = include_str!("../../passport.dat");
    let ppvec = input
        .split("\n\n")
        .map(|pp| Passport::new(pp))
        .collect::<Vec<_>>();

    let num_valid = ppvec.iter().filter(|pp| pp.is_valid_parta()).count();
    println!("aoc4a: {}", num_valid);

    let num_valid = ppvec.iter().filter(|pp| pp.is_valid_partb()).count();
    println!("aoc4b: {}", num_valid);
}
