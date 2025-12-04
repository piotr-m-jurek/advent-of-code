import gleam/list
import gleam/string

pub fn pretty_print_array(arr arr: List(a), fun fun: fn(a) -> String) {
  let string_of_array = arr |> list.map(fun) |> string.join(", ")
  "Array: [" <> string_of_array <> "]"
}

pub fn tuple_to_string(tuple tuple: #(String, String)) {
  let #(a, b) = tuple
  "#(" <> a <> ", " <> b <> ")"
}
