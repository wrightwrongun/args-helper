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

fn main() {
    let mut args = Args::new();
    let mut args = args
                    .required("name")
                    .required("file")
                    .optional("filter")
                    .flag("-d")
                    .flag("-v")
                    .flag("+b");
    
    match args.check() {
        Ok(args) => {
            let name = args.get_arg("name").unwrap();
            let file = args.get_arg("file").unwrap();
            if let Some(filter) = args.get_arg("filter") {
                println!("filter name is {}", filter);
            }
            else {
                println!("no filter");
            }

            let is_debug = args.has_flag("-d");
            let is_verbose = args.has_flag("-v");
            let is_background = args.has_flag("+b");

            // Do stuff..!

        },
        Err(e) => {
            eprintln!("usage: {}", args); // Prints the app's command-line.
        }
    }
}
