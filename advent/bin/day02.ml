open Core

let part_1_test = "./inputs/day2.txt"

let conditions =
  [ 12, Re.Perl.compile_pat "(\\d{1,}) red"
  ; 13, Re.Perl.compile_pat "(\\d{1,}) green"
  ; 14, Re.Perl.compile_pat "(\\d{1,}) blue"
  ]
;;

let () =
  Fmt.pr "\n";
  Fmt.pr "\n";
  let lines = Lib.read_all part_1_test in
  List.foldi lines ~init:0 ~f:(fun idx res line ->
    let matched =
      List.filter conditions ~f:(fun (limit, condition) ->
        let matches = Re.all condition line in
        List.find matches ~f:(fun m ->
          let str = Re.Group.get m 1 in
          Int.of_string str > limit)
        |> Option.is_some)
    in
    if List.length matched = 0 then res + idx + 1 else res)
  |> Fmt.pr "@part1: %d@."
;;
