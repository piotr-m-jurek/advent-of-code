use anyhow::Result;
use itertools::Itertools;

fn part_1() -> Result<()> {
    let input = include_str!("./day10.prod").trim();

    let mut x = 1;
    let mut counter = 1;
    let mut total = 0;

    for line in input.lines() {
        if counter % 40 == 20 {
            let add = counter * x;
            total += add;
            println!(
                "X: {}, total: {}, counter: {}, add: {}",
                x, total, counter, add
            );
        }
        counter += 1;

        if let Some(("addx", amount)) = line.split_once(" ") {
            if counter % 40 == 20 {
                let add = counter * x;
                total += add;
                println!(
                    "X: {}, total: {}, counter: {}, add: {}",
                    x, total, counter, add
                );
            }

            let amount: i32 = amount.parse().unwrap();
            x += amount;
            counter += 1;
        }
    }

    println!("Total: {} ", total);

    return Ok(());
}

const LIT: char = '█';
const DIM: char = ' ';

const COLS: usize = 40;
const ROWS: usize = 6;
const SPRITE_WIDTH: u32 = 3;

fn get_pixel(cycle: usize, x: i32) -> char {
    let current_col = (cycle - 1) % COLS;
    if (current_col as i32).abs_diff(x) <= SPRITE_WIDTH / 2 {
        return LIT;
    } else {
        return DIM;
    }
}

fn part_2() -> Result<()> {
    let input = include_str!("./day10.prod").trim();

    let mut x = 1;
    let mut counter = 1;
    let mut screen = [' '; COLS * ROWS];

    for line in input.lines() {
        screen[counter - 1] = get_pixel(counter, x);
        counter += 1;

        if let Some(("addx", amount)) = line.split_once(" ") {
            screen[counter - 1] = get_pixel(counter, x);
            let amount: i32 = amount.parse().unwrap();
            x += amount;
            counter += 1;
        }
    }
    let image = screen
        .chunks(COLS)
        .map(|row| row.iter().collect())
        .collect::<Vec<String>>()
        .join("\n");
    let _ = screen
        .chunks(COLS)
        .map(|row| {
            let row = row.iter().collect();
            println!("{:?}", row);
            return row;
        })
        .collect::<Vec<String>>()
        .join("\n");

    Ok(())
}

fn main() -> Result<()> {
    let _ = part_1();
    let _ = part_2();
    return Ok(());
}
