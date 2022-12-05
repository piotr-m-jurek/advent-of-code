use anyhow::Error;
fn main() -> Result<(), Error> {
    let input: usize = include_str!("day-3.txt")
        .trim()
        .lines()
        .map(|item| {
            let (first, second) = item.split_at(item.len());

            first
                .chars()
                .filter(|&char| second.contains(char))
                .take(1)
                .next()
                .unwrap()
        })
        .map(get_idx)
        .sum();
    dbg!(input);
    Ok(())
}

fn get_idx(letter: char) -> usize {
    if let Some(idx) = ('a'..='z')
        .into_iter()
        .chain(('A'..='Z').into_iter())
        .position(|c| c == letter)
    {
        idx + 1
    } else {
        panic!("At the disco")
    }
}
