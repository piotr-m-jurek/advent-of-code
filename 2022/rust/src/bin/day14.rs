use std::{
    collections::HashMap,
    fmt::{self},
    str::FromStr,
};

use anyhow::{Error, Result};

fn main() -> Result<()> {
    let input = include_str!("./day14.test");
    let mut cave = Cave::default();
    for line in input.lines() {
        let mut coords = line.split(" -> ");
        let mut start = coords.next().unwrap().parse()?;
        while let Some(end) = coords.next() {
            cave.draw_wall(&start, &end.parse()?)
        }
    }
    // let cave = Cave::default();
    return Ok(());
}

// ################
// ##### TILE #####
// ################

enum Tile {
    Rock,
    Sand,
    Air,
}
impl Tile {
    fn print(&self) -> &str {
        match self {
            Tile::Rock => "#",
            Tile::Sand => "o",
            Tile::Air => ".",
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

#[derive(Default)]
struct Cave {
    tiles: HashMap<Coord, Tile>,
}

impl Cave {
    fn draw_wall(&self, start: &Coord, end: &Coord) {}
}

// #################
// ##### COORD #####
// #################

#[derive(Debug, PartialEq, Eq)]
struct Coord {
    x: isize,
    y: isize,
}

impl FromStr for Coord {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").expect("extracting coords error");
        return Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
        });
    }
}
