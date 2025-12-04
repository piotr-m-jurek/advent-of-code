import day2
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
    // "998-1012",
  // "1188511880-1188511890",
  // "222220-222224",
  // "1698522-1698528",
  // "446443-446449",
  // "38593856-38593862",
  // "565653-565659",
  // "824824821-824824827",
  // "2121212118-212121212",
  ]
  assert day2.part2(ids) == 243
}
