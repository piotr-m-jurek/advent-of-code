import gleam/result
import gleam/string
import simplifile

pub fn read_lines(
  filepath: String,
) -> Result(List(String), simplifile.FileError) {
  simplifile.read(from: filepath)
  |> result.map(string.split(_, on: "\n"))
}

pub fn read_csv(filepath: String) {
  simplifile.read(from: filepath)
  |> result.map(fn(content) {
    content
    |> string.replace("\n", "")
    |> string.split(",")
  })
}
