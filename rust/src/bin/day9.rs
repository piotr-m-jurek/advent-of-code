use std::collections::HashSet;

use anyhow::Result;
use itertools::Itertools;

#[derive(Default, Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

fn main() -> Result<()> {
    part_1();
    part_2();
    return Ok(());
}
fn part_1() -> Result<()> {
    let input = include_str!("./day-9.prod");
    let mut seen: HashSet<Point> = HashSet::new();
    let starting_position = Point::default();
    let mut head = starting_position;
    let mut tail = starting_position;
    println!("{:?} {:?}", seen, starting_position);

    seen.insert(tail);

    for line in input.lines() {
        let (dir, amount) = line
            .split_once(" ")
            .expect("instructions should be or not be");
        let amount: usize = amount.parse().expect("the amount needs to be here");

        for _ in 0..amount {
            // move head to the position
            match dir {
                "U" => head.y -= 1,
                "D" => head.y += 1,
                "L" => head.x -= 1,
                "R" => head.x += 1,
                _ => panic!("this direction is not supported"),
            }
            let diff = Point {
                x: head.x - tail.x,
                y: head.y - tail.y,
            };

            if diff.x.abs() > 1 || diff.y.abs() > 1 {
                // catch up with tail if needed
                tail.x += diff.x.signum();
                tail.y += diff.y.signum();
                // update seen hashset
                seen.insert(tail);
            }
        }
    }
    println!("Part 1: {:?}", seen);
    println!("Part 1:  {} ", seen.len());

    return Ok(());
}

fn part_2() -> Result<()> {
    let input = include_str!("./day-9.prod");
    let mut seen: HashSet<Point> = HashSet::new();
    let starting_position = Point::default();
    let mut rope = [starting_position; 10];
    println!("{:?} {:?}", seen, starting_position);

    seen.insert(starting_position);

    for line in input.lines() {
        let (dir, amount) = line
            .split_once(" ")
            .expect("instructions should be or not be");
        let amount: usize = amount.parse().expect("the amount needs to be here");

        for _ in 0..amount {
            // move head to the position
            match dir {
                "U" => rope[0].y -= 1,
                "D" => rope[0].y += 1,
                "L" => rope[0].x -= 1,
                "R" => rope[0].x += 1,
                _ => panic!("this direction is not supported"),
            }
            for (head_idx, tail_idx) in (0..rope.len()).tuple_windows(){
                let diff = Point {
                    x: rope[head_idx].x - rope[tail_idx].x,
                    y: rope[head_idx].y - rope[tail_idx].y,
                };

                if diff.x.abs() > 1 || diff.y.abs() > 1 {
                    // catch up with rope[tail_idx] if needed
                    rope[tail_idx].x += diff.x.signum();
                    rope[tail_idx].y += diff.y.signum();
                    // update seen hashset
                    seen.insert(rope[rope.len() -1]);
                }

            };
        }
    }
    println!("Part 2: {:?}", seen);
    println!("Part 2:  {} ", seen.len());

    return Ok(());
}
