// AOC 2020 day 20

#[derive(Debug, PartialEq)]
enum Flip {
    EastWest,
    NorthSouth,
    None,
}

#[derive(Debug)]
struct Tile {
    id: usize,
    pattern: Vec<Vec<char>>,
    edges: Vec<Vec<u16>>,
    matches: Vec<Vec<(usize, bool, usize)>>, // (tile idx, reversed, side idx)
    orient: (usize, Flip),                   // (number of clock-wise rotations, type of flip)
}

impl Tile {
    fn new(id: usize, pattern: &Vec<Vec<char>>) -> Self {
        Tile {
            id,
            pattern: pattern.to_owned(),
            edges: Tile::get_edges(pattern),
            matches: vec![vec![]; 4],
            orient: (0, Flip::None),
        }
    }

    // convert a vector of chars to a u16
    fn convert(v: &Vec<char>) -> u16 {
        let mut retval = 0;
        for c in v {
            let d = match c {
                '#' => 1,
                _ => 0,
            };
            retval = retval * 2 + d;
        }
        retval
    }

    // document the edges (normal and reversed) as u16s for matching purposes
    fn get_edges(v: &Vec<Vec<char>>) -> Vec<Vec<u16>> {
        let mut retval: Vec<Vec<u16>> = vec![vec![]; 2];
        // regular edges (NESW)
        retval[0].push(Tile::convert(&v[0]));
        retval[0].push(Tile::convert(&(0..=9).map(|i| v[i][9]).collect()));
        retval[0].push(Tile::convert(&v[9]));
        retval[0].push(Tile::convert(&(0..=9).map(|i| v[i][0]).collect()));

        // reversed edges (NESW)
        retval[1].push(Tile::convert(&(0..=9).map(|i| v[0][9 - i]).collect()));
        retval[1].push(Tile::convert(&(0..=9).map(|i| v[9 - i][9]).collect()));
        retval[1].push(Tile::convert(&(0..=9).map(|i| v[9][9 - i]).collect()));
        retval[1].push(Tile::convert(&(0..=9).map(|i| v[9 - i][0]).collect()));

        retval
    }
}

// document what matches with each edge
fn record_matches(t: &mut Vec<Tile>) {
    for i in 0..t.len() {
        for j in 0..t.len() {
            if i != j {
                for idx_mee in 0..4 {
                    for idx_mer in 0..4 {
                        if t[j].edges[0][idx_mer] == t[i].edges[0][idx_mee] {
                            t[i].matches[idx_mee].push((j, false, idx_mer));
                        }
                        if t[j].edges[1][idx_mer] == t[i].edges[0][idx_mee] {
                            t[i].matches[idx_mee].push((j, true, idx_mer));
                        }
                        // warn about a situation that makes the puzzle harder to put together
                        if t[i].matches[idx_mee].len() > 1 {
                            println!("Warning: edge is matched more than once {:?}", t[i]);
                        }
                    }
                }
            }
        }
    }
}

// return a list of the corner tile's indices
fn find_corners(tiles: &Vec<Tile>) -> Vec<usize> {
    let mut corners: Vec<usize> = vec![];
    for (tidx, t) in tiles.iter().enumerate() {
        let mut num_unmatched = 0;
        for em in &t.matches {
            if em.is_empty() {
                num_unmatched += 1;
            }
        }
        match num_unmatched {
            0 | 1 => {}
            2 => corners.push(tidx),
            _ => panic!("tile with >2 unmatched edges found: {:?}", tiles[tidx]),
        }
    }
    corners
}

// put the puzzle together row by row
fn construct_puzzle(tiles: &mut Vec<Tile>, slen: usize, grid: &mut Vec<Vec<usize>>) {
    let mut cur_y = 0;
    let mut cur_x = 1;
    let mut match_to_left = true;
    loop {
        // place and orient current tile
        if match_to_left {
            // match the East side of mee to the West side of mer
            let mee = grid[cur_y][cur_x - 1];
            let (mee_rot, mee_flip) = &tiles[mee].orient;
            let east_idx = (5 - mee_rot + if *mee_flip == Flip::EastWest { 2 } else { 0 }) % 4;
            let (mer, match_reversed, side_matched) = tiles[mee].matches[east_idx][0];
            let mer_rot = 3 - side_matched;
            let mer_rev = if mer_rot == *mee_rot || (mer_rot + mee_rot) % 4 == 1 {
                match_reversed
            } else {
                !match_reversed
            };
            if mer_rev && *mee_flip != Flip::NorthSouth || !mer_rev && *mee_flip == Flip::NorthSouth
            {
                tiles[mer].orient = (mer_rot, Flip::NorthSouth);
            } else {
                tiles[mer].orient = (mer_rot, Flip::None);
            }
            grid[cur_y][cur_x] = mer;
        } else {
            // match the South side of mee to the North side of mer
            let mee = grid[cur_y - 1][cur_x];
            let (mee_rot, mee_flip) = &tiles[mee].orient;
            let south_idx = (6 - mee_rot + if *mee_flip == Flip::NorthSouth { 2 } else { 0 }) % 4;
            let (mer, match_reversed, side_matched) = tiles[mee].matches[south_idx][0];
            let mer_rot = (4 - side_matched) % 4;
            let mer_rev = if mer_rot == *mee_rot || mer_rot + mee_rot == 3 {
                match_reversed
            } else {
                !match_reversed
            };
            if mer_rev && *mee_flip != Flip::EastWest || !mer_rev && *mee_flip == Flip::EastWest {
                tiles[mer].orient = (mer_rot, Flip::EastWest);
            } else {
                tiles[mer].orient = (mer_rot, Flip::None);
            }
            grid[cur_y][cur_x] = mer;

            match_to_left = true;
        }

        cur_x += 1;
        if cur_x == slen {
            match_to_left = false;
            cur_x = 0;
            cur_y += 1;
            if cur_y == slen {
                break;
            }
        }
    }
}

// strip off tile edges, find sea serpents and finally count and return '#'s
fn solve_puzzle(tiles: &Vec<Tile>, slen: usize, grid: &Vec<Vec<usize>>) -> usize {
    let pslen = slen * 8;
    let mut ngrid: Vec<Vec<char>> = vec![vec![' '; pslen]; pslen];

    // create a grid with all tiles properly rotatad and flipped in place
    for y in 0..slen {
        for yi in 1..=8 {
            for x in 0..slen {
                let t = &tiles[grid[y][x]];
                let (rot, flip) = &t.orient;
                let mut xrev = *rot == 1 || *rot == 2;
                let mut yrev = *rot > 1;
                match *flip {
                    Flip::EastWest => xrev = !xrev,
                    Flip::NorthSouth => yrev = !yrev,
                    Flip::None => {}
                }
                let yidx = if yrev { 9 - yi } else { yi };
                for xi in 1..=8 {
                    let xidx = if xrev { 9 - xi } else { xi };
                    let (px, py) = if rot % 2 == 1 {
                        (yidx, xidx)
                    } else {
                        (xidx, yidx)
                    };
                    ngrid[y * 8 + yi - 1][x * 8 + xi - 1] = t.pattern[py][px];
                }
            }
        }
    }

    // template for sea serpent
    let ss: [(usize, usize); 15] = [
        (1, 0),
        (2, 1),
        (1, 5),
        (2, 4),
        (1, 6),
        (2, 7),
        (2, 10),
        (1, 11),
        (1, 12),
        (2, 13),
        (2, 16),
        (1, 17),
        (1, 18),
        (1, 19),
        (0, 18),
    ];

    // look for serpents in all 8 possible orientations, stopping when they are found
    for rot in 0..4 {
        for swap in 0..2 {
            let mut found = false;
            for y in 0..pslen - 2 {
                for x in 0..pslen - 19 {
                    let (xidx, xrev) = if rot < 2 {
                        (x, false)
                    } else {
                        (pslen - 1 - x, true)
                    };
                    let (yidx, yrev) = if rot % 3 == 0 {
                        (y, false)
                    } else {
                        (pslen - 1 - y, true)
                    };
                    let mut hits = 0;
                    for (yoff, xoff) in &ss {
                        let rx = if xrev { xidx - xoff } else { xidx + xoff };
                        let ry = if yrev { yidx - yoff } else { yidx + yoff };
                        let (px, py) = if swap == 1 { (ry, rx) } else { (rx, ry) };

                        // check against template
                        if ngrid[py][px] == '#' {
                            hits += 1;
                        } else {
                            break;
                        }
                    }
                    if hits == ss.len() {
                        found = true;
                        // update the grid to show the sea serpent
                        for (yoff, xoff) in &ss {
                            let rx = if xrev { xidx - xoff } else { xidx + xoff };
                            let ry = if yrev { yidx - yoff } else { yidx + yoff };
                            let (px, py) = if swap == 1 { (ry, rx) } else { (rx, ry) };
                            ngrid[py][px] = 'O';
                        }
                    }
                }
            }
            if found {
                // display the grid with sea serpents and stop looking
                for ni in ngrid.iter().take(pslen) {
                    println!("{}", (0..pslen).map(|x| ni[x]).collect::<String>());
                }
                break;
            }
        }
    }

    // calculate and return the water roughness (number of '#'s left)
    let mut retval = 0;
    for ni in ngrid.iter().take(pslen) {
        retval += ni.iter().filter(|&c| *c == '#').count();
    }
    retval
}

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut collecting_pattern = false;
    let mut pattern: Vec<Vec<char>> = vec![];
    let mut tiles: Vec<Tile> = vec![];
    let mut id: usize = 0;

    while reader.read_line(&mut inputstr)? != 0 {
        if inputstr.trim() == "" {
            collecting_pattern = false;
            tiles.push(Tile::new(id, &pattern));
            pattern.clear();
        } else if !collecting_pattern {
            id = inputstr.trim()[5..9].to_string().parse::<usize>().unwrap();
            collecting_pattern = true;
        } else {
            pattern.push(inputstr.trim().chars().collect::<Vec<char>>())
        }

        inputstr.clear();
    }

    // find corners by looking for tiles with 2 unmatched edges
    record_matches(&mut tiles);
    let corners = find_corners(&tiles);
    if corners.len() == 4 {
        let ansa: usize = corners.iter().map(|i| tiles[*i].id).product();
        println!("aoc20a: {}", ansa);
    }

    // put the puzzle together starting by orienting and placing the first corner
    let sidelen = (tiles.len() as f64).sqrt() as usize;
    let mut grid: Vec<Vec<usize>> = vec![vec![usize::MAX; sidelen]; sidelen];
    if tiles[corners[0]].matches[0].is_empty() && tiles[corners[0]].matches[1].is_empty() {
        tiles[corners[0]].orient = (3, Flip::None);
    } else if tiles[corners[0]].matches[1].is_empty() && tiles[corners[0]].matches[2].is_empty() {
        tiles[corners[0]].orient = (2, Flip::None);
    } else if tiles[corners[0]].matches[2].is_empty() && tiles[corners[0]].matches[3].is_empty() {
        tiles[corners[0]].orient = (1, Flip::None);
    } else if tiles[corners[0]].matches[3].is_empty() && tiles[corners[0]].matches[0].is_empty() {
        tiles[corners[0]].orient = (0, Flip::None);
    } else {
        panic!("Bad corner tile: {:?}", tiles[corners[0]])
    }
    grid[0][0] = corners[0];
    construct_puzzle(&mut tiles, sidelen, &mut grid);

    println!("aoc20b: {}", solve_puzzle(&tiles, sidelen, &grid));

    Ok(())
}
