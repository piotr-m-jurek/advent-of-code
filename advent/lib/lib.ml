open Core

let read_all filename =
  Stdio.In_channel.with_file filename ~f:(fun channel ->
    channel |> In_channel.input_all |> String.split_lines)
;;

let print_list_of_strs strs =
  Format.printf
    "\n%a \n"
    (Format.pp_print_list
       ~pp_sep:(fun fmt () -> Format.fprintf fmt "; \n")
       Format.pp_print_string)
    strs
;;
