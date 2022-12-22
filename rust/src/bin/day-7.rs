use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let input_2 = fs::read_to_string("./src/bin/day-7.prod")?;
    let required_amount = 100_000;
    let mut total = 0;

    let mut stack = vec![("/", 0)];
    for line in input_2.lines().filter(|line| !line.is_empty()) {
        if line == "$ cd /" || line == "$ ls" {
            continue;
        }

        if line.starts_with("$ cd ") {
            let dir = &line[5..];
            if dir == ".." {
                let (_, amount) = stack.pop().unwrap();
                if amount < required_amount {
                    total += amount;
                }
                stack.last_mut().unwrap().1 += amount;
            } else {
                stack.push((dir, 0));
                println!("pushing dir {} stack: {:?} ", dir, stack);
            }
        }

        let (amount, _) = line.split_once(" ").expect("this has to work here");
        if let Ok(amount) = amount.parse::<usize>() {
            stack.last_mut().unwrap().1 += amount;
        };
    }

    println!("Part 1: {}", total);

    return Ok(());
}
