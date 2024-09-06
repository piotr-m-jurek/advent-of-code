open Core

let test_filename = "inputs/day3-prod.txt"

type digit =
  { col_start : int
  ; col_end : int
  ; value : int
  ; row : int
  }
[@@deriving show]

type symbol =
  { value : char
  ; row : int
  ; col : int
  }
[@@deriving show]

let get_symbols row line =
  let char_list = String.to_list line in
  List.filter_mapi char_list ~f:(fun col el ->
    match el with
    | '\n' | '.' -> None
    | '0' .. '9' -> None
    | value -> Some { value; row; col })
;;

let collision (digit : digit) (symbol : symbol) =
  List.find Lib.directions ~f:(fun (x, y) ->
    let row = symbol.row + y in
    let col = symbol.col + x in
    digit.row = row && digit.col_end >= col && digit.col_start <= col)
  |> Option.is_some
;;

let get_digits row line =
  let make_part start finish =
    let value = Int.of_string (String.slice line start (finish + 1)) in
    { col_start = start; col_end = finish; value; row }
  in
  let rec aux start finish idx chars acc =
    match start, finish, chars with
    | None, None, '0' .. '9' :: rest -> aux (Some idx) (Some idx) (idx + 1) rest acc
    | None, None, _ :: rest -> aux None None (idx + 1) rest acc
    | Some start, _, '0' .. '9' :: rest -> aux (Some start) (Some idx) (idx + 1) rest acc
    | Some start, Some finish, _ :: rest ->
      let part = make_part start finish in
      aux None None (idx + 1) rest (part :: acc)
    | Some start, _, [] ->
      let part = make_part start (String.length line - 1) in
      part :: acc
    | _, _, [] -> acc
    | _ -> assert false
  in
  aux None None 0 (String.to_list line) []
;;

let () =
  Fmt.pr "@Day 3 @.";
  let input = test_filename |> Lib.read_lines in
  let symbols =
    List.foldi input ~init:[] ~f:(fun i acc line ->
      let line_symbols = get_symbols i line in
      acc @ line_symbols)
  in
  let all_digits =
    List.foldi input ~init:[] ~f:(fun i acc line ->
      let numbers = get_digits i line in
      acc @ numbers)
  in
  let all_collisions =
    List.fold all_digits ~init:0 ~f:(fun acc digit ->
      let collided = List.find symbols ~f:(collision digit) |> Option.is_some in
      if collided then digit.value + acc else acc)
  in
  Fmt.pr "@ Input @.";
  List.iter input ~f:(Fmt.pr "Input: %s @.");
  Fmt.pr "@ Part1: @.";
  all_collisions |> Fmt.pr "%d @.";
  Fmt.pr "@ Part 2: @.";
  let stars = symbols |> List.filter ~f:(fun symbol -> Char.equal symbol.value '*') in
  let star_collisions =
    List.fold stars ~init:0 ~f:(fun acc star ->
      let collided = all_digits |> List.filter ~f:(fun digit -> collision digit star) in
      if not (equal (List.length collided) 2)
      then acc
      else (
        let multiplied =
          collided |> List.fold ~init:1 ~f:(fun acc (digit : digit) -> acc * digit.value)
        in
        Fmt.pr "@ Collided:@.";
        multiplied + acc))
  in
  Fmt.pr "%d@." star_collisions;
  ()
;;
