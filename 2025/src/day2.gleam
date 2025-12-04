import file_helpers
import gleam/int
import gleam/io
import gleam/list
import gleam/order.{Eq}
import gleam/result
import gleam/string

pub fn main() {
  use ids <- result.try(file_helpers.read_csv("input/day2.test"))
  let part1 = part1(ids)
  let part2 = part2(ids)

  io.println("Part 1: " <> int.to_string(part1))
  io.println("Part 2: " <> int.to_string(part2))
  Ok(Nil)
}

// ========== Part 1 ==========

fn part1(ids: List(String)) {
  ids
  |> list.flat_map(parse_id)
  |> list.filter_map(check_for_validity)
  |> list.fold(0, int.add)
}

fn check_for_validity(num: Int) {
  let str = num |> int.to_string()
  let len = string.length(str)
  let middle = len / 2
  let first = string.slice(str, at_index: 0, length: middle)
  let second = string.slice(str, at_index: middle, length: len)
  case string.compare(first, second) {
    Eq -> Ok(num)
    _ -> Error(Nil)
  }
}

// =============================

// ========== Part 2 ==========

pub fn part2(ids) {
  ids
  |> list.flat_map(parse_id)
  |> list.filter(invalid)
  |> list.fold(0, int.add)
}

fn invalid(id) {
  let id_str = int.to_string(id)
  let id_len = string.length(id_str)
  let sizes = list.range(1, id_len / 2)
  list.any(sizes, check_for_size(_, id_str))
}

pub fn check_for_size(size, id_str) {
  case string.length(id_str) % size == 0, chunk_string(id_str, size) {
    True, [first, ..rest] -> list.all(rest, fn(chunk) { chunk == first })
    _, [] -> False
    _, _ -> False
  }
}

// ========== HELPERS ==========

pub fn chunk_string(s: String, len: Int) {
  s
  |> string.to_graphemes()
  |> list.sized_chunk(into: len)
  |> list.map(string.join(_, ""))
}

fn parse_id(id: String) -> List(Int) {
  case string.split(id, on: "-") {
    [first, second] -> {
      let assert Ok(parsed_first) = int.parse(first)
      let assert Ok(parsed_second) = int.parse(second)
      list.range(parsed_first, parsed_second)
    }
    _ -> []
  }
}
