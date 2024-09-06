open Core

let test_input = "./inputs/day1-test.txt"
let prod_input = "./inputs/day1-prod.txt"

module Translation = struct
  let cases =
    [ "one", "1"
    ; "two", "2"
    ; "three", "3"
    ; "four", "4"
    ; "five", "5"
    ; "six", "6"
    ; "seven", "7"
    ; "eight", "8"
    ; "nine", "9"
    ; "1", "1"
    ; "2", "2"
    ; "3", "3"
    ; "4", "4"
    ; "5", "5"
    ; "6", "6"
    ; "7", "7"
    ; "8", "8"
    ; "9", "9"
    ]
  ;;

  let substr_to_digit str pos =
    List.find_map cases ~f:(fun (substr, value) ->
      match String.substr_index ~pos str ~pattern:substr with
      | Some matched when matched = pos -> Some value
      | _ -> None)
  ;;

  let str_to_numbers str =
    let map_to_number = substr_to_digit str in
    List.range 0 (String.length str) |> List.filter_map ~f:map_to_number
  ;;
end

(* ================ *)
(* === SOLUTIONS=== *)
(* ================ *)

let part_1 lines =
  List.fold lines ~init:0 ~f:(fun acc line ->
    let chars = String.to_list line in
    let numbers = chars |> List.filter ~f:Char.is_digit in
    let combined_str = Fmt.str "%c%c" (List.hd_exn numbers) (List.last_exn numbers) in
    let number = Int.of_string combined_str in
    acc + number)
;;

let part_2 lines =
  List.map lines ~f:(fun line ->
    line
    |> Translation.str_to_numbers
    |> fun string_numbers -> String.concat ~sep:"" string_numbers)
  |> part_1
;;

(* PRINT *)
let () = Lib.read_lines test_input |> part_1 |> Fmt.pr "\nPart 1: %d"
let () = Lib.read_lines test_input |> part_2 |> Fmt.pr "\nPart 2: %d"
let () = Lib.read_lines prod_input |> part_2 |> Fmt.pr "\nbig input: %d"
