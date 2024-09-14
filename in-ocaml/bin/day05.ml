open Core
open Lib.A

type range =
  { dest : int
  ; source : int
  ; range : int
  }
[@@deriving show]

type puzzle =
  { seeds : int list
  ; maps : range list list
  }
[@@deriving show]

let parse_seeds = string "seeds: " *> sep_by1 whitespace digit

let parse_map_range =
  let* dest = wmatch digit in
  let* source = wmatch digit in
  let* range = wmatch digit in
  return { dest; source; range }
;;

let parse_map =
  let* _ = take_till (fun char -> Char.(char = '\n')) in
  sep_by1 newline parse_map_range
;;

let parse_puzzle =
  let* seeds = parse_seeds in
  let* _ = whitespace in
  let* maps = sep_by1 (string "\n\n") parse_map in
  return { seeds; maps }
;;

let mapper_for_seed seed (ranges : range list) =
  let matching =
    List.find ranges ~f:(fun range ->
      range.source <= seed && range.source + range.range >= seed)
  in
  match matching with
  | Some range -> seed - range.source + range.dest
  | None -> seed
;;

let map_seeds seed maps =
  List.fold maps ~init:seed ~f:(fun seed map -> mapper_for_seed seed map)
;;

let parse_input input =
  parse_string ~consume:Prefix parse_puzzle input |> Result.ok_or_failwith
;;

let () =
  Fmt.pr "Day 5: @.";
  ()
;;

let () =
  let input = Lib.read_all "inputs/day5-test.txt" in
  let { seeds; maps } = parse_input input in
  let result =
    List.fold seeds ~init:Int.max_value ~f:(fun acc seed ->
      let loc = map_seeds seed maps in
      min acc loc)
  in
  Fmt.pr "@ Part 1: %d @." result;
  ()
;;

let rec pair_seeds acc = function
  | start :: range :: rest -> (start, range) :: pair_seeds acc rest
  | [] -> acc
  | _ -> assert false
;;

let () =
  let input = Lib.read_all "inputs/day5-test.txt" in
  let { seeds; maps } = parse_input input in
  let seeds = pair_seeds [] seeds in
  List.iter seeds ~f:(fun (seed, range) -> Fmt.pr "seeds (%d * %d)" seed range);
  let result =
    List.fold seeds ~init:Int.max_value ~f:(fun acc (start, count) ->
      let local =
        List.range start (start + count)
        |> List.fold ~init:acc ~f:(fun acc seed ->
          let next_loc = map_seeds seed maps in
          Fmt.pr "@ Inner fold: acc %d seed %d next_loc %d @." acc seed next_loc;
          min acc next_loc)
      in
      min acc local)
  in
  Fmt.pr "@.==> part 2: %d@." result;
  ()
;;
