use std::collections::HashMap;

use anyhow::Result;

type Point = (usize, usize);

#[derive(Default, Debug)]
struct Map {
    grid: Vec<Vec<u8>>,
    start: Vec<Point>,
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
            map.start.push((row, start_point));
            gridline[start_point] = b'a';
        }
        if let Some(end_point) = gridline.iter().position(|&c| c == b'E') {
            map.end.0 = row;
            map.end.1 = end_point;
            gridline[end_point] = b'z';
        }
        map.grid.push(gridline);
    }
    map.height = map.grid.len();
    map.width = map.grid[0].len();
    return map;
}

fn main() -> Result<()> {
    let map = parse();
    let mut shortest_list: Vec<usize> = Vec::new();
    let mut to_visit: Vec<Point> = Vec::new();

    for start in map.start {
        let mut shortest: HashMap<Point, usize> = HashMap::new();
        shortest.insert(start, 0);
        to_visit.extend(get_possible_routes(start, map.width, map.height));

        while let Some(loc) = to_visit.pop() {
            let cur_elevation = map.grid[loc.0][loc.1];

            let points = get_possible_routes(loc, map.width, map.height);
            let valid = points
                .iter()
                .filter(|pos| map.grid[pos.0][pos.1] + 1 >= cur_elevation)
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

        let shortest = shortest.get(&map.end).unwrap();
        println!("Shortest path is: {} ", shortest);

        shortest_list.push(*shortest);
    }

    let global_shortest = shortest_list.iter().min().unwrap();
    println!("Global shortest = {:?}", global_shortest);

    return Ok(());
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
        .collect::<Vec<Point>>();
}
