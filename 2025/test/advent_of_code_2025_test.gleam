import day2
import day3
import gleeunit

pub fn main() {
  gleeunit.main()
}

pub fn chunk_string_test() {
  assert day2.chunk_string("123", 2) == ["12", "3"]
  assert day2.chunk_string("1234", 1) == ["1", "2", "3", "4"]
  assert day2.chunk_string("1234", 2) == ["12", "34"]
}

pub fn check_for_size_test() {
  assert day2.check_for_size(2, "1212") == True
  assert day2.check_for_size(3, "123123") == True
  assert day2.check_for_size(2, "1234") == False
  assert day2.check_for_size(1, "1111") == True
}

pub fn part2_test() {
  let ids = [
    "11-22",
    "95-115",
  ]
  assert day2.part2(ids) == 243
}

pub fn day3_part1_test() {
  let line = "463"
  assert day3.process_line_2(line, [], 2) == 63
}
