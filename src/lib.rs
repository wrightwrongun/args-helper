/* ----------------------------------------------------------------------------

    MIT License

    Copyright (c) 2024 MW

    Permission is hereby granted, free of charge, to any person obtaining a
    copy of this software and associated documentation files (the "Software"),
    to deal in the Software without restriction, including without limitation
    the rights to use, copy, modify, merge, publish, distribute, sublicense,
    and/or sell copies of the Software, and to permit persons to whom the
    Software is furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in
    all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
    FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
    DEALINGS IN THE SOFTWARE.

---------------------------------------------------------------------------- */

#![allow(dead_code, unused)]

mod args;
mod tests;

use args::Args;

// fn main() {
//     let line = ["abc", "def", "-a", "lmn", "pqr", "xyz", "+x"].iter().map(|x| String::from(*x)).collect();
//     // let line = ["abc"].iter().map(|x| String::from(*x)).collect();

//     let mut args = Args::init_with(line);
//     let mut args = args
//                 .require("one")
//                 .require("two")
//                 .require("three")
//                 .optional("four")
//                 .optional("five")
//                 .flag("-a")
//                 .flag("+x");
    
//     match args.check() {
//         Ok(args) => {
//             println!("arg--> one = '{:?}'", args.get_arg("one"));
//             println!("arg--> two = '{:?}'", args.get_arg("two"));
//             println!("arg--> three = '{:?}'", args.get_arg("three"));
//             println!("arg--> four = '{:?}'", args.get_arg("four"));
//             println!("arg--> five = '{:?}'", args.get_arg("five"));
//             println!("arg--> six = '{:?}'", args.get_arg("six"));
//             println!("flag-> -a = {}", args.has_flag("-a"));
//             println!("flag-> +x = {}", args.has_flag("+x"));
//             println!("flag-> -z = {}", args.has_flag("-z"));
//             },
//         Err(e) => {
//             eprintln!("args-error: {:?}", e);
//         }
//     }
// }
