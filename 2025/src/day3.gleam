import file_helpers
import gleam/int
import gleam/io
import gleam/list
import gleam/order
import gleam/otp/factory_supervisor
import gleam/result
import gleam/string

pub fn main() {
  io.println("Day 3 Begins now")
  let assert Ok(ids) = file_helpers.read_lines("input/day3.prod")
  let part1 = part1(ids)
  let part2 = part2(ids)

  io.println("Part 1: " <> int.to_string(part1))
  io.println("Part 2: " <> int.to_string(part2))
  Ok(Nil)
}

// ========== Part 1 ==========

fn part1(lines: List(String)) {
  lines
  |> list.map(process_line)
  |> list.fold(0, int.add)
}

pub fn process_line(line: String) {
  let res =
    line
    |> string.to_graphemes()
    |> list.combinations(2)
    |> list.map(list_to_int)
    |> list.max(int.compare)
  case res {
    Ok(num) -> num
    Error(_) -> panic as "No combinations found"
  }
}

// ========== Part 2 ==========

fn part2(lines: List(String)) {
  lines
  |> list.map(process_line_2(_, [], 12))
  |> list.fold(0, int.add)
}

pub fn process_line_2(line, acc, window) {
  case window, int.compare(string.length(line), window) {
    0, _ -> {
      list.reverse(acc) |> list.map(int.to_string) |> list_to_int()
    }
    _, order.Eq -> {
      let acc =
        line
        |> string.to_graphemes()
        |> list.reverse()
        |> list.map(int.parse)
        |> result.values
        |> list.append(acc)

      list.reverse(acc) |> list.map(int.to_string) |> list_to_int()
    }
    _, order.Gt -> {
      let cutout = string.length(line) - window + 1

      let assert Ok(first_biggest) =
        string.slice(line, at_index: 0, length: cutout)
        |> string.to_graphemes()
        |> list.map(int.parse)
        |> result.values
        |> list.max(int.compare)

      let rest =
        line
        |> string.to_graphemes()
        |> list.drop_while(less(_, first_biggest))
        |> list.rest()
        |> result.unwrap([])
        |> string.join("")

      process_line_2(rest, [first_biggest, ..acc], window - 1)
    }
    _, _ -> list.reverse(acc) |> list.map(int.to_string) |> list_to_int()
  }
}

// ========== HELPERS ==========

fn list_to_int(ints: List(String)) -> Int {
  case ints |> string.join("") |> int.base_parse(10) {
    Ok(num) -> num
    Error(_) -> 0
  }
}

fn less(c, big) {
  int.parse(c)
  |> result.unwrap(0)
  |> int.compare(big)
  == order.Lt
}
