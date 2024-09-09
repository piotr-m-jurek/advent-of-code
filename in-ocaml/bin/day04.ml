open Core
open Angstrom

let filename = "inputs/day4-prod.txt"

module IntSet = Stdlib.Set.Make (Int)

type digits =
  { winning : int list
  ; ours : int list
  }

let digit = take_while1 Char.is_digit >>| Int.of_string <?> "Couldn't parse digit"
let whitespace = take_while Char.is_whitespace
let wstring str = whitespace *> string str <* whitespace

let parse_card =
  let* _ = string "Card" *> whitespace *> digit <* wstring ":" in
  let* winning = sep_by1 whitespace digit in
  let* _ = wstring "|" in
  let* ours = sep_by1 whitespace digit in
  return { winning; ours }
;;

let parse_line str =
  Fmt.pr "@ Line: %s " str;
  match parse_string ~consume:Prefix parse_card str with
  | Ok parsed -> parsed
  | Error e -> Fmt.failwith "failed to parse line: %s" e
;;

let () = Fmt.pr "@ Day 4: @."

let () =
  Fmt.pr "@ Day 4: @.";
  let input = Lib.read_lines filename in
  let part1_score =
    List.fold input ~init:0 ~f:(fun acc line ->
      let { winning; ours } = parse_line line in
      let winning = IntSet.of_list winning in
      let ours = IntSet.of_list ours in
      let intersection = IntSet.cardinal @@ IntSet.inter winning ours in
      match intersection with
      | 0 -> acc
      | cardinal -> acc + Int.pow 2 (cardinal - 1))
  in
  Fmt.pr "@ Part 1: %d @." part1_score;
  ()
;;

let () =
  let input = Lib.read_lines filename in
  let scores =
    List.fold input ~init:[] ~f:(fun acc line ->
      let { winning; ours } = parse_line line in
      let winning = IntSet.of_list winning in
      let ours = IntSet.of_list ours in
      let intersection = IntSet.cardinal @@ IntSet.inter winning ours in
      intersection :: acc)
    |> List.rev
  in
  let multipliers = Array.create ~len:(List.length input) 1 in
  List.iteri scores ~f:(fun idx score ->
    List.range 1 (score + 1)
    |> List.iter ~f:(fun offset ->
      let card = offset + idx in
      multipliers.(card) <- multipliers.(card) + multipliers.(idx)));
  Array.to_list multipliers |> List.iter ~f:(Fmt.pr "el: %d, ");
  Fmt.pr "@ Part 2: %d @." (Array.reduce multipliers ~f:( + ) |> Option.value_exn);
  ()
;;
