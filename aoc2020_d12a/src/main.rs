// AOC 2020 day 12 problem 1

#[derive(Debug, Clone, Copy)]
enum Direction {
    East,
    West,
    North,
    South,
}

#[derive(Debug)]
struct Ship {
    dir: Direction,
    x: i32,
    y: i32,
}

impl Ship {
    fn new(dir: Direction) -> Self {
        Self { dir, x: 0, y: 0 }
    }

    fn update(&mut self, control: char, amt: i32) {
        match control {
            'N' => self.update_location(Direction::North, amt),
            'S' => self.update_location(Direction::South, amt),
            'E' => self.update_location(Direction::East, amt),
            'W' => self.update_location(Direction::West, amt),
            'F' => self.update_location(self.dir, amt),
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

    fn update_location(&mut self, d: Direction, amt: i32) {
        match d {
            Direction::East => self.x += amt,
            Direction::North => self.y += amt,
            Direction::West => self.x -= amt,
            Direction::South => self.y -= amt,
        }
    }

    fn update_direction(&mut self, control: char) {
        match control {
            'L' => match self.dir {
                Direction::East => self.dir = Direction::North,
                Direction::North => self.dir = Direction::West,
                Direction::West => self.dir = Direction::South,
                Direction::South => self.dir = Direction::East,
            },
            'R' => match self.dir {
                Direction::East => self.dir = Direction::South,
                Direction::North => self.dir = Direction::East,
                Direction::West => self.dir = Direction::North,
                Direction::South => self.dir = Direction::West,
            },
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
    let mut ship = Ship::new(Direction::East);

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

    println!("aoc12a: {}", ship.distance_from_origin());

    Ok(())
}
