import gleam/result
import gleam/string
import simplifile

pub fn read_lines(filepath: String) {
  simplifile.read(from: filepath)
  |> result.map(string.split(_, on: "\n"))
}
