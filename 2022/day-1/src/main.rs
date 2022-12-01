use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut parsed_and_computed: Vec<u32> = read_input()
        .split(|el| el.is_empty())
        .map(|food| {
            food.iter()
                .map(|v| {
                    let failed_msg = format!("couldn't parse one of the inputs {}", v);
                    v.parse::<u32>().expect(&failed_msg)
                })
                .sum()
        })
        .collect();
    parsed_and_computed.sort();

    let task1 = parsed_and_computed.clone().pop().unwrap_or(0);
    let task2: u32 = parsed_and_computed.iter().rev().take(3).sum();

    println!("Task 1 answer: {:?}", task1);
    println!("Task 2 answer: {:?}", task2);

    return Ok(());
}

fn read_input() -> Vec<String> {
    let raw = fs::read_to_string("src/input.txt")
        .expect("couldn't read input file")
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    raw
}
