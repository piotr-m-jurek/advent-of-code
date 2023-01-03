use anyhow::Result;
use std::str::FromStr;

#[derive(Debug)]
struct Move {
    what: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut raw = s.split_whitespace().flat_map(|ch| ch.parse::<usize>());
        Ok(Move {
            what: raw.next().expect("must exist"),
            from: raw.next().expect("must exist") - 1,
            to: raw.next().expect("Must exist") - 1,
        })
    }
}

#[derive(Debug, Clone)]
struct Crane(Vec<String>);

impl FromStr for Crane {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let max_stacks = s
            .clone()
            .chars()
            .last()
            .expect("There must be a number of all columns")
            .to_string()
            .parse::<usize>()?;
        let mut outcome: Vec<Vec<char>> = vec![];
        for _ in 0..max_stacks {
            outcome.push(vec![]);
        }

        println!(" max_stacks: {}", max_stacks);
        for line in s.lines().rev().skip(1) {
            for (stack_idx_raw, item) in line.chars().enumerate() {
                if stack_idx_raw % 4 == 1 && item != ' ' {
                    let stack_idx = (stack_idx_raw - 1) / 4;
                    outcome[stack_idx].push(item);
                }
            }
        }
        return Ok(Crane(
            outcome.iter().map(|rows| rows.iter().collect()).collect(),
        ));
    }
}

fn part1(mut stacks: Crane, commands: &Vec<Move>) -> String {
    for &Move { what, from, to } in commands {
        for _ in 0..what {
            if let Some(item) = stacks.0[from].pop() {
                stacks.0[to].push(item)
            }
        }
    }
    println!("after: {:?}", stacks);

    let mut res = String::new();
    for mut stack in stacks.0 {
        if let Some(item) = stack.pop() {
            res.push(item);
        }
    }

    return res;
}

fn part2(mut stacks: Crane, commands: &Vec<Move>) -> String {
    for &Move { what, from, to } in commands {
        let chars: String = stacks.0[from].chars().rev().take(what).collect();

        for _ in 0..chars.len() {
            stacks.0[from].pop();
        }
        for char in chars.chars().rev() {
            stacks.0[to].push(char)
        }
    }
    println!("after: {:?}", stacks);
    let mut res = String::new();
    for mut stack in stacks.0 {
        if let Some(item) = stack.pop() {
            res.push(item);
        }
    }

    return res;
}

fn main() -> Result<()> {
    let input = include_str!("./day5.prod");
    let (stacks_raw, commands_raw) = input.split_once("\n\n").expect("aoc should provide");
    let stacks = stacks_raw.parse::<Crane>()?;
    let commands: Vec<Move> = commands_raw
        .lines()
        .flat_map(|line| line.parse::<Move>())
        .collect();

    // println!("Part 1: {}", part1(stacks.clone(), &commands));
    println!("Part 2: {}", part2(stacks.clone(), &commands));
    Ok(())
}
