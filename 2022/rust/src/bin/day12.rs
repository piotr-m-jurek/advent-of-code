use std::collections::HashMap;

use anyhow::Result;

type Point = (usize, usize);

#[derive(Default, Debug)]
struct Map {
    grid: Vec<Vec<u8>>,
    start: Point,
    starts: Vec<Point>,
    end: Point,
    height: usize,
    width: usize,
}

fn parse() -> Map {
    let input = include_str!("./day12.prod");
    let mut map = Map::default();

    for (row, line) in input.lines().enumerate() {
        let mut gridline = line.chars().map(|c| c as u8).collect::<Vec<_>>();
        if let Some(start_point) = gridline.iter().position(|&c| c == b'S' || c == b'a') {
            map.start = (row, start_point);
            map.starts.push((row, start_point));
            gridline[start_point] = b'a';
        }
        if let Some(end_point) = gridline.iter().position(|&c| c == b'E') {
            map.end = (row, end_point);
            gridline[end_point] = b'z';
        }
        map.grid.push(gridline);
    }
    map.height = map.grid.len();
    map.width = map.grid[0].len();

    return map;
}

fn main() -> Result<()> {
    let part_1 = part_1();
    let part_2 = part_2();
    println!("Part 1 = {part_1:?}");
    println!("Part 2 = {part_2:?}");

    return Ok(());
}

fn part_2() -> Option<usize> {
    let parsed  = parse();
    let Map {
        start: _start,
        end,
        width,
        height,
        grid,
        starts,
    } = parsed;
    let mut shortest_list: Vec<usize> = Vec::new();

    for start in starts {
        if let Some(shortest) = get_shortest_path(start, end, &grid, width, height) {
            shortest_list.push(shortest);
        }
    }

    return shortest_list.iter().min().copied();
}

fn part_1() -> Option<usize> {
    let parsed  = parse();
    let Map {
        start,
        end,
        width,
        height,
        grid,
        starts: _starts,
    } = parsed;

    return get_shortest_path(start, end, &grid, width, height);
}

fn get_possible_routes(start: Point, width: usize, height: usize) -> Vec<Point> {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let istart = (start.0 as i32, start.1 as i32);
    let iheight = height as i32;
    let iwidth = width as i32;

    return directions
        .iter()
        .map(|pos| (istart.0 + pos.0, istart.1 + pos.1))
        .filter(|pos| pos.0 >= 0 && pos.1 >= 0 && pos.0 < iheight && pos.1 < iwidth)
        .map(|(x, y)| (x as usize, y as usize))
        .collect();
}

fn get_shortest_path(
    start: Point,
    end: Point,
    grid: &Vec<Vec<u8>>,
    width: usize,
    height: usize,
) -> Option<usize> {
    let mut shortest: HashMap<Point, usize> = HashMap::new();
    let mut to_visit: Vec<Point> = Vec::new();

    shortest.insert(start, 0);
    to_visit.extend(get_possible_routes(start, width, height));

    while let Some(loc) = to_visit.pop() {
        let cur_elevation = grid[loc.0][loc.1];

        let points = get_possible_routes(loc, width, height);
        let valid = points
            .iter()
            .filter(|pos| grid[pos.0][pos.1] + 1 >= cur_elevation)
            .copied()
            .collect::<Vec<Point>>();

        let new_path_dist = valid.iter().filter_map(|pos| shortest.get(pos)).min();

        if new_path_dist.is_none() {
            continue;
        }

        let new_path_dist = new_path_dist.unwrap() + 1;
        let curr_path_dist = shortest.entry(loc).or_insert(usize::MAX);

        if *curr_path_dist > new_path_dist {
            *curr_path_dist = new_path_dist;
            to_visit.extend(valid.iter());
        }
    }

    return shortest.get(&end).copied();
}
