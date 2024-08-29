open Core

let test_input = "./inputs/day2-prod.txt"

let conditions =
  [ 12, Re.Perl.compile_pat "(\\d{1,}) red"
  ; 13, Re.Perl.compile_pat "(\\d{1,}) green"
  ; 14, Re.Perl.compile_pat "(\\d{1,}) blue"
  ]
;;

let () =
  let lines = Lib.read_all test_input in
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
  |> Fmt.pr "@ Part 1: %d @."
;;

let () =
  Fmt.pr "@ Part 2: @.";
  let matchers =
    [ Re.Perl.compile_pat "(\\d{1,}) red"
    ; Re.Perl.compile_pat "(\\d{1,}) green"
    ; Re.Perl.compile_pat "(\\d{1,}) blue"
    ]
  in
  let lines = Lib.read_all test_input in
  let res =
    List.fold lines ~init:0 ~f:(fun acc line ->
      let result =
        List.map matchers ~f:(fun matcher ->
          let matches = Re.all matcher line in
          List.fold matches ~init:1 ~f:(fun acc m ->
            let num = Re.Group.get m 1 |> Int.of_string_opt |> Option.value_exn in
            max acc num))
      in
      acc + List.fold result ~init:1 ~f:( * ))
  in
  Fmt.pr "@ Part 2: %d@." res;
  ()
;;
