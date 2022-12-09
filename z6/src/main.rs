use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    zad1(&input);
    zad2(&input);
}

fn zad1(input: &str) {
    let mut counter = 4;
    for (a1, a2, a3, a4) in input.chars().tuple_windows() {
        if ![a2, a3, a4].contains(&a1)
            && ![a1, a3, a4].contains(&a2)
            && ![a1, a2, a4].contains(&a3)
            && ![a1, a2, a3].contains(&a4)
        {
            println!("position: {} -> {}{}{}{}", counter, a1, a2, a3, a4);
            break;
        }
        counter += 1
    }
}
fn zad2(input: &str) -> Option<()> {
    let mut counter = 14;
    for ab in input.chars().collect::<Vec<char>>().windows(14) {
        let mut hash_map: HashMap<&char, usize> = HashMap::new();
        for value in ab.into_iter() {
            hash_map.insert(value, hash_map.get(value).unwrap_or(&0) + 1);
        }
        let mut did_change = false;
        for (key, value) in hash_map {
            if value > 1 {
                did_change = true;
            }
        }
        if did_change == false {
            println!("message at: {}", counter);
            return Some(());
        }
        counter += 1
    }
    None
}
