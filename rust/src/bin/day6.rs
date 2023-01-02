use anyhow::Result;

fn main() -> Result<()> {
    let src: Vec<char> = include_str!("./day-6.prod").trim().chars().collect();

    let part1 = find_first_unique_x(&src, 4);
    println!("Part1 {}", part1);

    let part2 = find_first_unique_x(&src, 14);
    println!("Part2 {:?}", part2);

    return Ok(());
}

fn find_first_unique_x(chars: &Vec<char>, window: usize) -> usize {
    return chars
        .windows(window)
        .position(|x| are_unique(&String::from_iter(x), window))
        .expect("at least one")
        + window;
}

fn are_unique(str: &str, window_size: usize) -> bool {
    let mut y = str.chars().collect::<Vec<char>>();

    y.sort();
    y.dedup();

    return y.len() == window_size;
}
