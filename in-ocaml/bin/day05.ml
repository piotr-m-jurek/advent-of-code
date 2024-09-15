open Core
open Lib.A

type range =
  { dest : int
  ; source : int
  ; length : int
  }
[@@deriving show]

type puzzle =
  { seeds : int list
  ; maps : range list list
  }
[@@deriving show]

(*
   PARSING
*)

let parse_seeds = string "seeds: " *> sep_by1 space digit

let parse_map_range =
  let* dest = wmatch digit in
  let* source = wmatch digit in
  let* length = digit in
  return { dest; source; length }
;;

let parse_map =
  let* _ = take_till (fun ch -> Char.(ch = '\n')) in
  sep_by1 newline parse_map_range
;;

let puzzle =
  let* seeds = parse_seeds in
  let* _ = whitespace in
  let* maps = sep_by1 (string "\n\n") parse_map in
  return { seeds; maps }
;;

let parse_input input = parse_string ~consume:Prefix puzzle input

(*
   MAPPING
*)

let mapper_for_seed seed (ranges : range list) =
  let matching =
    List.find ranges ~f:(fun range ->
      range.source <= seed && range.source + range.length >= seed)
  in
  match matching with
  | Some range -> seed - range.source + range.dest
  | None -> seed
;;

let map_seeds seed maps = List.fold maps ~init:seed ~f:mapper_for_seed

let () =
  Fmt.pr "Day 5: @.";
  ()
;;

(* let () = *)
(*   let input = Lib.read_all "inputs/day5-test.txt" in *)
(*   Fmt.pr "@ Input: \n %s" input; *)
(*   let input = parse_input input in *)
(*   let game = *)
(*     match input with *)
(*     | Ok { seeds; maps } -> { seeds; maps } *)
(*     | _ -> assert false *)
(*   in *)
(*   game |> Fmt.pr "%a" pp_puzzle; *)
(*   Fmt.pr "Maps length: %d" (List.length game.maps); *)
(*   List.iteri game.maps ~f:(fun idx map -> *)
(*     Fmt.pr "@ Map: %d" idx; *)
(*     List.iter map ~f:(Fmt.pr "@ Range: %a" pp_range)); *)
(*   let result = *)
(*     List.fold game.seeds ~init:Int.max_value ~f:(fun acc seed -> *)
(*       let loc = map_seeds seed game.maps in *)
(*       min acc loc) *)
(*   in *)
(*   Fmt.pr "@ Part 1: %d @." result; *)
(*   () *)
(* ;; *)

let rec pair_seeds acc = function
  | start :: range :: rest -> (start, range) :: pair_seeds acc rest
  | [] -> acc
  | _ -> assert false
;;

let () =
  let input = Lib.read_all "inputs/day5-prod.txt" in
  let input = parse_input input in
  let game =
    match input with
    | Ok { seeds; maps } -> { seeds; maps }
    | _ -> assert false
  in
  let seeds = pair_seeds [] game.seeds in
  let result =
    List.fold seeds ~init:Int.max_value ~f:(fun acc (start, count) ->
      let local =
        List.range start (start + count)
        |> List.fold ~init:acc ~f:(fun acc seed -> map_seeds seed game.maps |> min acc)
      in
      min acc local)
  in
  Fmt.pr "@.==> part 2: %d@." result;
  ()
;;
