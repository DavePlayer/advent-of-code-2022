use std::{collections::HashMap, path::PathBuf};

fn main() {
    let input = std::fs::read_to_string("./data.txt").unwrap();
    let mut sizes = HashMap::new();
    let mut affected = Vec::new();

    for line in input.lines() {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        let parts: Vec<_> = line.split_whitespace().collect();
        match parts[..] {
            ["$", "cd", ".."] => {
                affected.pop();
            }
            ["$", "cd", name] => {
                affected.push(name);
            }
            [size, _] => {
                let size: u32 = size.parse().unwrap();
                for idx in 0..affected.len() {
                    let path = PathBuf::from_iter(&affected[..=idx]);
                    *sizes.entry(path).or_insert(0) += size;
                }
            }
            _ => {}
        };
    }
    for (key, value) in sizes.iter() {
        println!("{key:?} -> {value}");
    }
    let ans: u32 = sizes.values().filter(|size| *size <= &100000).sum();
    println!("ans: {ans}");

    // zad 2
    let disk_size = 70000000;
    let remove_atleast = 30000000;
    let root = sizes.get(&PathBuf::from("/")).unwrap();
    let available = disk_size - root;

    let removable: HashMap<PathBuf, u32> = sizes
        .into_iter()
        .filter(|(path, size)| available + size >= remove_atleast)
        .collect();
    println!("{removable:?}");
    let smallest = removable.values().min().unwrap();
    println!("smallest: {smallest}");
}
