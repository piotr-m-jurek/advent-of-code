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
