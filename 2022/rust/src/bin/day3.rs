use anyhow::Error;
fn main() -> Result<(), Error> {
    let lines = include_str!("day3.txt")
        .trim()
        .split('\n')
        .collect::<Vec<&str>>();

    let task1: usize = lines
        .iter()
        .map(|item| {
            let (first, second) = item.split_at(item.len() / 2);

            get_repeating_letter(first, second)
                .expect("Get Repeating Letter didn't work at some ponit")
        })
        .map(|letter| get_idx(letter).unwrap())
        .sum();

    println!("Task 1: {}", task1);

    let task2: usize = lines
        .chunks(3)
        .map(|three_lines| {
            get_repeating_letter2(three_lines[0], three_lines[1], three_lines[2]).unwrap()
        })
        .map(|letter| get_idx(letter).unwrap())
        .sum();

    println!("Task2: {}", task2);
    Ok(())
}

fn get_idx(input: char) -> Option<usize> {
    let lowercase = ('a'..='z').into_iter();
    let uppercase = ('A'..='Z').into_iter();

    lowercase
        .chain(uppercase)
        .position(|char| char == input)
        .map(|x| x + 1)
}

fn get_repeating_letter2(first: &str, second: &str, third: &str) -> Option<char> {
    first
        .chars()
        .filter(|&char| second.contains(char) && third.contains(char))
        .take(1)
        .next()
}

fn get_repeating_letter(first: &str, second: &str) -> Option<char> {
    first
        .chars()
        .filter(|&char| second.contains(char))
        .take(1)
        .next()
}
