open Core
open Lib.A

type input =
  { timings : int list
  ; distances : int list
  }
[@@deriving show]

let parse_input =
  let* timings =
    string "Time:" *> skip_while Char.is_whitespace *> sep_by whitespace digit <* newline
  in
  let* distances =
    string "Distance:" *> skip_while Char.is_whitespace *> sep_by whitespace digit
  in
  return { timings; distances }
;;

type input_part_2 =
  { time : int
  ; distance : int
  }
[@@deriving show]

let parse_part_2 =
  let* time =
    string "Time:" *> skip_while Char.is_whitespace *> sep_by whitespace digit <* newline
  in
  let* distance =
    string "Distance:" *> skip_while Char.is_whitespace *> sep_by whitespace digit
  in
  let time = time |> List.map ~f:string_of_int |> String.concat |> int_of_string in
  let distance =
    distance |> List.map ~f:string_of_int |> String.concat |> int_of_string
  in
  return { time; distance }
;;

let calculate_score (time, distance) =
  let timings = List.range 0 time in
  List.filter timings ~f:(fun hold ->
    let calculation = (time - hold) * hold in
    calculation > distance)
  |> List.length
;;

let () =
  Fmt.pr "@ === Day 6: === @.";
  ()
;;

let () =
  let input = Lib.read_all "inputs/day-6-prod.txt" in
  let input = parse_string ~consume:Prefix parse_input input in
  let input =
    match input with
    | Ok { timings; distances } -> { timings; distances }
    | Error e -> Fmt.failwith "Couldn't parse input with error %s" e
  in
  Fmt.pr "@ Input: %s @." (show_input input);
  let coupled = List.zip_exn input.timings input.distances in
  let result =
    List.fold coupled ~init:1 ~f:(fun acc race ->
      let race_result = calculate_score race in
      race_result * acc)
  in
  Fmt.pr "@ Part 1: %d @." result;
  ()
;;

let () =
  let input = Lib.read_all "inputs/day-6-prod.txt" in
  let input = parse_string ~consume:Prefix parse_part_2 input in
  let input =
    match input with
    | Ok { time; distance } -> { time; distance }
    | Error e -> Fmt.failwith "Couldn't parse input with error %s" e
  in
  Fmt.pr "@ Input: %s @." (show_input_part_2 input);
  let race_result = calculate_score (input.time, input.distance) in
  Fmt.pr "@ Part 2: %d @." race_result;
  ()
;;
