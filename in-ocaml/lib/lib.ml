open! Core

let read_lines = In_channel.read_lines
let read_all = In_channel.read_all

let print_list_of_strs strs =
  Format.printf
    "\n%a \n"
    (Format.pp_print_list
       ~pp_sep:(fun fmt () -> Format.fprintf fmt "; \n")
       Format.pp_print_string)
    strs
;;

let directions = [ 0, 1; 1, 1; 1, 0; 1, -1; 0, -1; -1, -1; -1, 0; -1, 1 ]

let range_seq start range =
  let next i = if i > range then None else Some (i, i + 1) in
  Seq.unfold next start
;;

module A = struct
  include Angstrom

  let is_newline = function
    | '\n' -> true
    | _ -> false
  ;;

  let whitespace = take_while Char.is_whitespace
  let digit = take_while1 Char.is_digit >>| Int.of_string <?> "Couldn't parse digit"
  let newline = take_while is_newline
  let string = string
  let wmatch matcher = whitespace *> matcher <* whitespace
end
