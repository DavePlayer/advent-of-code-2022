fn main() {
    let path = "./data.txt";
    let data = std::fs::read_to_string(path);
    if let Ok(data) = data {
        let input = data.split("\n").filter(|l| l != &"").collect::<Vec<&str>>();
        let input = input
            .iter()
            .map(|tpl| tpl.split_once(",").unwrap_or(("not", "working")))
            .map(|tpl| {
                (
                    (
                        tpl.0.split_once("-").unwrap().0.parse::<usize>().unwrap(),
                        tpl.0.split_once("-").unwrap().1.parse::<usize>().unwrap(),
                    ),
                    (
                        tpl.1.split_once("-").unwrap().0.parse::<usize>().unwrap(),
                        tpl.1.split_once("-").unwrap().1.parse::<usize>().unwrap(),
                    ),
                )
            })
            .collect::<Vec<((usize, usize), (usize, usize))>>();
        zad2(input);
    }
}

fn zad1(input: Vec<((usize, usize), (usize, usize))>) {
    let mut counter = 0;
    for ranges in input {
        if (ranges.0 .0 <= ranges.1 .0 && ranges.1 .1 <= ranges.0 .1)
            || (ranges.1 .0 <= ranges.0 .0 && ranges.0 .1 <= ranges.1 .1)
        {
            counter += 1;
            // println!(
            //     "{}..{} {}..{} overlap",
            //     ranges.0 .0, ranges.0 .1, ranges.1 .0, ranges.1 .1
            // )
        }
    }
    println!("{} pairs overlap", counter);
}
fn zad2(input: Vec<((usize, usize), (usize, usize))>) {
    let mut counter = 0;
    for ranges in input {
        if ranges.0 .0 <= ranges.1 .1 && ranges.1 .0 <= ranges.0 .1 {
            counter += 1;
            // println!(
            //     "{}..{} {}..{} overlap",
            //     ranges.0 .0, ranges.0 .1, ranges.1 .0, ranges.1 .1
            // )
        }
    }
    println!("{} pairs overlap", counter);
}
