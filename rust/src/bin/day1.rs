fn main() {
    let mut parsed_and_computed: Vec<u32> = read_input()
        .split(|el| el.is_empty())
        .map(|food| food.iter().map(|v| v.parse::<u32>().unwrap()).sum())
        .collect();

    parsed_and_computed.sort();

    let task1 = parsed_and_computed.clone().pop().unwrap();
    let task2: u32 = parsed_and_computed.iter().rev().take(3).sum();

    println!("Task 1 answer: {:?}", task1);
    println!("Task 2 answer: {:?}", task2);
}

fn read_input() -> Vec<String> {
    std::fs::read_to_string("src/bin/day1.txt")
        .expect("couldn't read input file")
        .lines()
        .map(|x| x.to_string())
        .collect()
}
