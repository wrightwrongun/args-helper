# args-helper
**Rust** - Simple parser for command-line arguments.

## About
A simple utility library for simplifying the use of command-line arguments in **Rust** console applications.

## Build
Grab the repository, and build with *Cargo*:
```
    git clone https://github.com/wrightwrongun/args-helper
    cd args-helper
    cargo build
    cargo test
```

## Usage
Usually used at the beginning of `main()`.e.g.
```rust
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
                    println!("filter is '{}'", filter);
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
```

### Running
If running the above code, with a *console* app called *'hello-world'* -
1. With enough command-line arguments:
```
    hello-world kylie stuff.txt pretty
```
2. Will result in printing:
```
    filter is 'pretty'
```
1. Without enough command-line arguments:
```
    hello-world
```
2. Will result in printing:
```
    usage: hello-world <name> <file> [filter] [-v] [-d] [+b]
```

## Notes
- Field values are read from the command-line in the order that they are specified.
- It is considered an error if there are not enough command-line arguments to populate every `required` field.
- A call to `.check()` will return an  `Ok` if all required fields have been populated by command-line arguments, otherwise it returns  `Err()`. 
- Panics if an `optional` field is specified after a `required` field is specified. e.g.
```rust
    args
    .required("one")
    .optional("two")
    .required("three")  // <--- Panics here because of preceding line!
    .optional("four");
```
- Panics if the same field name is specified twice, regardless of whether it is `required` or `optional`. e.g.
```rust
    args
    .required("abc")
    .required("xyz")
    .optional("abc");   // <--- Panics here because of repeated name!
```
- A command-line argument beginning with `-` or `+` is considered to be a flag, and is ignored when populating `required` and `optional` fields.
- All flags are considered optional.
- Flags can be specified before or after`required` or `optional` fields. e.g.
```rust
    args
    .required("abc")
    .flag("-x")
    .required("pqr")
    .flag("+y")
    .optional("xyz")
    .flag("-z);
```
is considered to be the same as:

```rust
    args
    .required("abc")
    .required("pqr")
    .optional("xyz")
    .flag("+y")
    .flag("-x")
    .flag("-z);
```
- Flags do not have to be specified using `.flag(name)` before a call to `.has_flag(name)`. The call to `.flag(name)` is used to build the usage string used in printing `args` to the console.

## ToDo
- [x] Parse arguments
- [x] Output usage information to console
- [ ] Add argument descriptions for enhanced usage information
- [ ] Add functionality for required flags.

## Help
For suggestions, improvements, or job offers, message me at [wrightwrongun](https://github.com/wrightwrongun).