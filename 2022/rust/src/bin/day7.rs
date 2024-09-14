use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let input_2 = fs::read_to_string("./src/bin/day7.prod")?;

    let required_amount_part_1 = 100_000;
    let mut total_part_1 = 0;

    let mut final_countdown = vec![];

    let mut stack = vec![("/", 0)];
    for line in input_2.lines().filter(|line| !line.is_empty()) {
        if line == "$ cd /" || line == "$ ls" {
            continue;
        }

        if line.starts_with("$ cd ") {
            let dir = &line[5..];
            if dir == ".." {
                let (name, amount) = stack.pop().unwrap();
                if amount < required_amount_part_1 {
                    total_part_1 += amount;
                }
                stack.last_mut().unwrap().1 += amount;
                final_countdown.push((name, amount));
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

    while stack.len() > 0 {
        let (name, amount) = stack.pop().unwrap();
        final_countdown.push((name, amount));

        if stack.len() > 0 {
            stack.last_mut().unwrap().1 += amount;
        }
    }

    let total_space = 70_000_000;
    let unused_required = 30_000_000;
    let free_space = total_space - final_countdown.last().unwrap().1;
    let space_to_delete = unused_required - free_space;

    println!(
        "total {}, free {}, to_delete {}",
        total_space, free_space, space_to_delete
    );

    let answer2 = final_countdown
        .into_iter()
        .filter(|(_, amount)| *amount >= space_to_delete)
        .map(|(name, amount)| {
            println!("{}: {}", name, amount);
            return amount;
        })
        .min();

    println!("Stack {:?}", stack);
    println!("Part 1: {}", total_part_1);
    println!("Part 2: {:?}", answer2);

    return Ok(());
}
