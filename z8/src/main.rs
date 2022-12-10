fn main() {
    let input = std::fs::read_to_string("./data.txt").unwrap();
    let data = input
        .split("\n")
        .map(|line| {
            line.split("")
                .filter(|c| *c != "")
                .map(|c| c.parse::<u16>().unwrap_or(0))
                .collect::<Vec<u16>>()
        })
        .collect::<Vec<Vec<u16>>>();
    // zad1(data);
    zad2(data);
}

fn zad2(data: Vec<Vec<u16>>) {
    let mut scores: Vec<i64> = vec![];
    for i in 1..data.len() - 1 {
        for j in 1..data[i].len() - 1 {
            let element = data[i][j];

            let mut top_trees = 0;
            let mut bottom_trees = 0;
            let mut left_trees = 0;
            let mut right_trees = 0;

            // top
            for k in (0..i).rev() {
                top_trees += 1;
                if data[k][j] >= element {
                    break;
                }
            }

            // bottom
            for k in i + 1..data.len() {
                bottom_trees += 1;
                if data[k][j] >= element {
                    break;
                }
            }

            // left
            for k in (0..j).rev() {
                left_trees += 1;
                if data[i][k] >= element {
                    break;
                }
            }

            // right
            for k in j + 1..data[i].len() {
                right_trees += 1;
                if data[i][k] >= element {
                    break;
                }
            }
            println!(
                "[{}][{}] = {} * {} * {} * {} = {}",
                i,
                j,
                top_trees,
                left_trees,
                right_trees,
                bottom_trees,
                (top_trees * bottom_trees * left_trees * right_trees)
            );
            scores.push((top_trees * bottom_trees * left_trees * right_trees));
        }
    }
    println!("max: {}", scores.iter().max().unwrap());
}

fn zad1(data: Vec<Vec<u16>>) {
    let mut invisible = 0;
    let mut visible = 0;
    for i in 1..data.len() - 1 {
        for j in 1..data[i].len() - 1 {
            let element = data[i][j];

            let mut top_visible = true;
            let mut bottom_visible = true;
            let mut left_visible = true;
            let mut right_visible = true;

            // top
            for k in 0..i {
                if data[k][j] >= element {
                    top_visible = false;
                    break;
                }
            }

            // bottom
            for k in i + 1..data.len() {
                if data[k][j] >= element {
                    bottom_visible = false;
                    break;
                }
            }

            // left
            for k in 0..j {
                if data[i][k] >= element {
                    left_visible = false;
                    break;
                }
            }

            // right
            for k in j + 1..data[i].len() {
                if data[i][k] >= element {
                    right_visible = false;
                    break;
                }
            }
            if !right_visible && !bottom_visible && !left_visible && !top_visible {
                // println!(
                //     "[{i}][{j}] is invisible {} {} {} {}",
                //     top_visible, bottom_visible, right_visible, left_visible
                // );
                invisible += 1;
            } else {
                // println!(
                //     "[{i}][{j}] is visible {} {} {} {}",
                //     top_visible, bottom_visible, right_visible, left_visible
                // );
                visible += 1;
            }
        }
    }
    visible += 4 * (data.len() - 1);
    println!("visible: {visible}");
    println!("invisible: {invisible}");
}
