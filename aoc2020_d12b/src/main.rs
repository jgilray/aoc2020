// AOC 2020 day 12 problem 2

#[derive(Debug)]
struct Ship {
    x: i32,
    y: i32,
    wpx: i32,
    wpy: i32,
}

impl Ship {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            wpx: 10,
            wpy: 1,
        }
    }

    fn update(&mut self, control: char, amt: i32) {
        match control {
            'N' => self.wpy += amt,
            'S' => self.wpy -= amt,
            'E' => self.wpx += amt,
            'W' => self.wpx -= amt,
            'F' => self.update_location(amt),
            'R' | 'L' => {
                if amt % 90 != 0 {
                    panic!("bad amt {}", amt);
                } else if amt > 0 {
                    self.update_direction(control);
                    self.update(control, amt - 90);
                }
            }
            _ => {}
        }
    }

    fn update_location(&mut self, mul: i32) {
        self.x += self.wpx * mul;
        self.y += self.wpy * mul;
    }

    fn update_direction(&mut self, control: char) {
        match control {
            'L' => {
                let tmpx = self.wpx;
                self.wpx = -self.wpy;
                self.wpy = tmpx;
            }
            'R' => {
                let tmpx = self.wpx;
                self.wpx = self.wpy;
                self.wpy = -tmpx;
            }
            _ => {}
        }
    }

    fn distance_from_origin(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut ship = Ship::new();

    while reader.read_line(&mut inputstr)? != 0 {
        let cntrl = inputstr.trim().chars().next().expect("bad control char");
        let amt: i32 = inputstr
            .trim()
            .chars()
            .skip(1)
            .collect::<String>()
            .parse()
            .expect("bad amt string");
        ship.update(cntrl, amt);
        inputstr.clear();
    }

    println!("aoc12b: {}", ship.distance_from_origin());

    Ok(())
}
