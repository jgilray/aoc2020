// AOC 2020 day 21

use std::collections::{HashMap, HashSet};

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();

    // map of allergen names to a vec of ingredient lists
    let mut hma: HashMap<String, Vec<HashSet<String>>> = HashMap::new();

    // map of ingredients -> number of appearances of each
    let mut safe_ingredients: HashMap<String, usize> = HashMap::new();

    while reader.read_line(&mut inputstr)? != 0 {
        let instr = inputstr.trim().trim_end_matches(')').to_string();
        let mut split_top = instr.split(" (contains ");
        let hsi: HashSet<String> = split_top
            .next()
            .unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();
        let allergens: Vec<String> = split_top
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.to_string())
            .collect();

        // collect ingredients and the number of times used
        for i in &hsi {
            let num = safe_ingredients.entry(i.clone()).or_insert(0);
            *num += 1;
        }

        // collect allergens and all ingredient lists they occur in
        for a in allergens {
            let i_vec = hma.entry(a).or_insert_with(Vec::new);
            i_vec.push(hsi.clone());
        }

        inputstr.clear();
    }

    // for each allergen intersect the ingredient lists to reduce the number of possibles
    let mut nhma: HashMap<String, HashSet<String>> = HashMap::new();
    for (a, vi) in &hma {
        let mut hs = vi[0].clone();
        for i_list in vi.iter().skip(1) {
            hs = hs.intersection(&i_list).cloned().collect::<HashSet<_>>();
        }
        nhma.insert(a.to_owned(), hs);
    }

    // iterate over the allergen -> possible ingredients map filling a map of allergen -> ingredient
    let mut hmai: HashMap<String, String> = HashMap::new();
    loop {
        let mut ingredient = "".to_string();
        let mut found = false;

        // find first uniquely determined allergen, ingredient pair
        for (a, hsi) in &nhma {
            if hsi.len() == 1 {
                ingredient = hsi.iter().last().unwrap().clone();
                hmai.insert(a.clone(), ingredient.clone());
                safe_ingredients.remove(&ingredient);
                found = true;
                break;
            }
        }

        // if found then remove unique ingredient from all lists, else we are done
        if found {
            for (_, hsi) in nhma.iter_mut() {
                hsi.remove(&ingredient);
            }
        } else {
            break;
        }
    }

    // print the number of times the safe ingredients appear in all lists
    println!("aoc21a: {}", safe_ingredients.values().sum::<usize>());

    // use the allergen -> ingredient map to create a string of dangerous ingredients
    let mut unsafe_ingredients: Vec<_> = hmai.iter().map(|(a, i)| (a.clone(), i.clone())).collect();
    unsafe_ingredients.sort();
    let mut dl: String = "".to_string();
    for (_, di) in &unsafe_ingredients {
        dl = dl + di + ",";
    }
    println!("aoc21b: {}", dl.trim_end_matches(','));

    Ok(())
}
