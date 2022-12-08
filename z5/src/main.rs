use regex;

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn main() {
    let path = "./data.txt";
    let input_data = std::fs::read_to_string(path).unwrap();

    let (crates_arangment, commands) = input_data.split_once("\n\n").unwrap();

    let commands = commands.split("\n").collect::<Vec<&str>>();

    let (stacks_str, _) = crates_arangment.rsplit_once('\n').unwrap();

    let mut rows: Vec<Vec<Option<char>>> = vec![];
    for line in stacks_str.lines() {
        let mut row: Vec<Option<char>> = vec![];
        for i in (1..line.len()).step_by(4) {
            let character = line.chars().nth(i).unwrap();
            // println!("|{}|", character);
            if character == ' ' {
                row.push(None);
            } else {
                row.push(Some(character));
            }
        }
        rows.push(row);
    }
    let mut rows = transpose(rows);

    let commands = commands
        .iter()
        .map(|x| {
            regex::Regex::new("move |from |to ")
                .unwrap()
                .split(x)
                .collect::<Vec<&str>>()
                .into_iter()
                .filter(|x| x.len() > 0)
                .map(|l| l.trim().parse::<usize>().unwrap_or(100000))
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();
    zad2(commands, rows);
}

fn zad2(commands: Vec<Vec<usize>>, mut rows: Vec<Vec<Option<char>>>) {
    for command in commands.iter() {
        let (amount, from, to) = (command[0], command[1], command[2]);
        let mut buffer: Vec<Option<char>> = vec![];
        let mut iterator = 0;
        for elements in rows[from - 1].iter_mut() {
            if let Some(element) = elements {
                if iterator < amount {
                    buffer.push(Some(*element));
                    *elements = None;
                    iterator += 1
                } else {
                    iterator = 0;
                    break;
                }
            }
        }
        // buffer.reverse();
        buffer.extend(rows[to - 1].iter().filter(|x| x.is_some()));
        rows[to - 1] = buffer;
        println!("");
        rows.iter().for_each(|x| println!("{:?}", x));
        println!("");
    }
    let mut ans: String = "".into();
    for column in rows.iter() {
        for value in column.iter() {
            if let Some(value) = value {
                ans.push(*value);
                break;
            }
        }
    }
    println!("odp: {}", ans);
}

fn zad1(commands: Vec<Vec<usize>>, mut rows: Vec<Vec<Option<char>>>) {
    for command in commands.iter() {
        let (amount, from, to) = (command[0], command[1], command[2]);
        let mut buffer: Vec<Option<char>> = vec![];
        let mut iterator = 0;
        for elements in rows[from - 1].iter_mut() {
            if let Some(element) = elements {
                if iterator < amount {
                    buffer.push(Some(*element));
                    *elements = None;
                    iterator += 1
                } else {
                    iterator = 0;
                    break;
                }
            }
        }
        buffer.reverse();
        buffer.extend(rows[to - 1].iter().filter(|x| x.is_some()));
        rows[to - 1] = buffer;
        println!("");
        rows.iter().for_each(|x| println!("{:?}", x));
        println!("");
    }
    let mut ans: String = "".into();
    for column in rows.iter() {
        for value in column.iter() {
            if let Some(value) = value {
                ans.push(*value);
                break;
            }
        }
    }
    println!("odp: {}", ans);
}
