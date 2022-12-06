use anyhow::Result;
use core::str::FromStr;

struct Task {
    start: usize,
    end: usize,
}

impl Task {
    pub fn overlaps(&self, other: &Task) -> bool {
        return self.start <= other.start && self.end >= other.start
            || self.end >= other.end && self.start <= other.end;
    }

    pub fn contains(&self, other: &Task) -> bool {
        return self.start <= other.start && self.end >= other.end;
    }
}

impl FromStr for Task {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .split_once("-")
            .expect("well if it's not that i don't know what");
        return Ok(Task {
            start: left.parse()?,
            end: right.parse()?,
        });
    }
}

struct Tasks {
    left: Task,
    right: Task,
}
impl Tasks {
    pub fn contains(&self) -> bool {
        return self.left.contains(&self.right) || self.right.contains(&self.left);
    }
    pub fn overlaps(&self) -> bool {
        return self.left.overlaps(&self.right) || self.right.overlaps(&self.left);
    }
}

impl FromStr for Tasks {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(",").expect("wrong aoc input");
        return Ok(Tasks {
            left: left.parse()?,
            right: right.parse()?,
        });
    }
}

fn main() -> Result<()> {
    let data = include_str!("day-4.prod").trim().lines();

    let task1 = data
        .clone()
        .flat_map(|line| line.parse::<Tasks>())
        .filter(|tasks| tasks.contains())
        .count();
    let task2 = data
        .flat_map(|line| line.parse::<Tasks>())
        .filter(|tasks| tasks.overlaps())
        .count();

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);

    // dbg!(&data);
    // let buddies = data
    //     .iter()
    //     .map(|str| {
    //         str.split(',').map(|buddies| {
    //             dbg!(buddies.split("").take(1).next());
    //             return buddies;
    //         })
    //     })
    //     .collect::<Vec<_>>();
    // dbg!(buddies);
    Ok(())
}
