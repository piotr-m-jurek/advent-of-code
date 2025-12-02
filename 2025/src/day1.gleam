import file_helpers
import gleam/int
import gleam/io
import gleam/order.{Eq, Gt, Lt}
import gleam/result

pub fn main() {
  use lines <- result.try(file_helpers.read_lines("input/day1.prod.txt"))

  io.println("Part 1: " <> int.to_string(solve(lines, 50, 0, False)))
  io.println("Part 2: " <> int.to_string(solve(lines, 50, 0, True)))
  Ok(Nil)
}

fn parse_line(line: String) {
  case line {
    "L" <> num -> int.parse(num) |> result.map(int.negate)
    "R" <> num -> int.parse(num)
    _ -> Error(Nil)
  }
}

fn solve(input: List(String), pos: Int, count: Int, part2: Bool) -> Int {
  case input {
    [] -> count
    [first, ..rest] -> {
      let assert Ok(delta) = parse_line(first)
      let new_pos = { pos + delta } % 100
      let new_count = case part2 {
        True -> count + count_crossings(pos, pos + delta)
        False ->
          case pos {
            0 -> count + 1
            _ -> count
          }
      }
      solve(rest, new_pos, new_count, part2)
    }
  }
}

fn count_crossings(from: Int, to: Int) -> Int {
  let crosses_zero = case int.compare(from, 0), int.compare(to, 0) {
    Gt, Lt | _, Eq -> 1
    _, _ -> 0
  }
  int.absolute_value(to) / 100 + crosses_zero
}
