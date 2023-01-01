use anyhow::Result;

fn directions(grid: &[Vec<u32>], x: usize, y: usize) -> [Vec<u32>; 4] {
    let row = grid[y].clone();
    let column: Vec<u32> = grid.iter().map(|row| row[x]).collect();
    let (left, right) = row.split_at(x);
    let (up, down) = column.split_at(y);
    let up = up.iter().copied().rev().collect();
    let left = left.iter().copied().rev().collect();

    let right = right[1..].to_vec();
    let down = down[1..].to_vec();
    return [up, right, left, down];
}

fn main() -> Result<()> {
    let input = include_str!("./day-8.prod");
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().flat_map(|c| c.to_digit(10)).collect())
        .collect();

    let len = grid.len();
    let cartesian_product = (1..len - 1).flat_map(|y| (1..len - 1).map(move |x| (x, y)));

    let part1 = cartesian_product
        .clone()
        .map(|(y, x)| {
            let height = grid[y][x];
            directions(&grid, x, y)
                .iter()
                .map(|direction| direction.iter().all(|h| *h < height))
                .any(|visible| visible)
        })
        .filter(|visible| *visible)
        .count()
        + (len - 1) * 4;

    let part2 = cartesian_product
        .map(|(x, y)| {
            let height = grid[y][x];
            directions(&grid, x, y)
                .iter()
                .map(|direction| {
                    direction
                        .iter()
                        .position(|h| *h >= height)
                        .map(|p| p + 1)
                        .unwrap_or_else(|| direction.len())
                })
                .product::<usize>()
        })
        .max()
        .unwrap();
    println!("Part1: {part1}");
    println!("Part2: {part2}");

    return Ok(());
}
