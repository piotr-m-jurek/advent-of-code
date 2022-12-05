/*
 *
 * A - rock
 * B - paper
 * C - scissors
 *
 * X - rock / 1 point
 * Y - paper / 2 points
 * Z - scissors / 3 points
 *
 * lose - 0
 * draw - 3
 * win -- 6


const GAMES = {
  AX: 4,
  AY: 8,
  AZ: 3,
  BX: 1,
  BY: 5,
  BZ: 9,
  CX: 7,
  CY: 2,
  CZ: 6,
};

const GAMES_2 = {
  AX: 3,
  AY: 4,
  AZ: 8,
  BX: 1,>
  BY: 5,
  BZ: 9,
  CX: 2,
  CY: 6,
  CZ: 7,
};

const arr = readFile("./data.txt").map((el) => el.replace(" ", ""));

const score = arr.reduce((prev, curr) => prev + GAMES[curr], 0);
const score2 = arr.reduce((prev, curr) => prev + GAMES_2[curr], 0);

console.log(score, score2);
 */

fn main() {
    let something = include_str!("day-2.txt")
        .trim()
        .lines()
        .map(|strat| strat.replace(" ", ""));
    println!(
        "task1 : {}",
        something
            .clone()
            .map(|element| match_score_task_1(&element))
            .sum::<usize>()
    );
    println!(
        "task1 : {}",
        something
            .map(|element| match_score_task_2(&element))
            .sum::<usize>()
    );
}

fn match_score_task_1(game: &str) -> usize {
    match game {
        "AX" => 4,
        "AY" => 8,
        "AZ" => 3,
        "BX" => 1,
        "BY" => 5,
        "BZ" => 9,
        "CX" => 7,
        "CY" => 2,
        "CZ" => 6,
        _ => panic!("not implemented"),
    }
}

fn match_score_task_2(game: &str) -> usize {
    match game {
        "AX" => 3,
        "AY" => 4,
        "AZ" => 8,
        "BX" => 1,
        "BY" => 5,
        "BZ" => 9,
        "CX" => 2,
        "CY" => 6,
        "CZ" => 7,
        _ => panic!("not implemented"),
    }
}
