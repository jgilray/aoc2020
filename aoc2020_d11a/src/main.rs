// AOC 2020 day 11

use std::cmp::min;

struct SeatingArea {
    grid: Vec<Vec<char>>,
    vacancy_num: u32,
}

impl SeatingArea {
    fn new(v: Vec<Vec<char>>, vacancy_num: u32) -> Self {
        Self {
            grid: v,
            vacancy_num,
        }
    }

    // iterates once according to the rules (like game of life), returning number of changes made
    fn iter_one(&mut self) -> u32 {
        let current_grid: Vec<Vec<char>> = self.grid.clone();
        let mut changes = 0;
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                match self.grid[y][x] {
                    'L' => {
                        if self.vacancy_num == 4 && count_adjacent(&current_grid, x, y) == 0
                            || self.vacancy_num == 5 && count_visible(&current_grid, x, y) == 0
                        {
                            self.grid[y][x] = '#';
                            changes += 1;
                        }
                    }
                    '#' => {
                        if self.vacancy_num == 4 && count_adjacent(&current_grid, x, y) >= 4
                            || self.vacancy_num == 5 && count_visible(&current_grid, x, y) >= 5
                        {
                            self.grid[y][x] = 'L';
                            changes += 1;
                        }
                    }
                    _ => {}
                }
            }
        }

        changes
    }

    // iterates until the grid no longer changes, returning the number of occupied seats at that point
    fn iter_until_stable(&mut self) -> u32 {
        while self.iter_one() > 0 {}

        let mut count: u32 = 0;
        for vc in self.grid.iter() {
            count += vc
                .iter()
                .map(|c| if *c == '#' { 1 } else { 0 })
                .sum::<u32>()
        }
        count
    }
}

// returns the number of occupied seats adjacent to the passed location
fn count_adjacent(v: &[Vec<char>], x: usize, y: usize) -> u32 {
    let mut occ = 0;

    let ylim = min(y + 1, v.len() - 1);
    for yy in y.saturating_sub(1)..=ylim {
        let xlim = min(x + 1, v[y].len() - 1);
        for xx in x.saturating_sub(1)..=xlim {
            if (yy != y || xx != x) && v[yy][xx] == '#' {
                occ += 1;
            }
        }
    }

    occ
}

// returns the number of occupied seats visible from the passed location
fn count_visible(v: &[Vec<char>], x: usize, y: usize) -> u32 {
    let dir = vec![
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];
    let mut occ = 0;

    // look out from x,y in each direction until off the grid or hit an L or #
    for d in &dir {
        for mult in 1.. {
            let tx = (x as isize) + d.0 * mult;
            let ty = (y as isize) + d.1 * mult;
            if tx < 0
                || ty < 0
                || tx as usize >= v[y].len()
                || ty as usize >= v.len()
                || v[ty as usize][tx as usize] == 'L'
            {
                break;
            } else if v[ty as usize][tx as usize] == '#' {
                occ += 1;
                break;
            }
        }
    }

    occ
}

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut v: Vec<Vec<char>> = vec![];

    while reader.read_line(&mut inputstr)? != 0 {
        v.push(inputstr.trim().chars().collect::<Vec<_>>());
        inputstr.clear();
    }

    let mut s1 = SeatingArea::new(v.clone(), 4);
    println!("aoc11a: {}", s1.iter_until_stable());
    let mut s2 = SeatingArea::new(v, 5);
    println!("aoc11b: {}", s2.iter_until_stable());

    Ok(())
}
