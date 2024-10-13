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

use std::collections::{HashMap, HashSet};
use std::env;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::ops::Index;

pub enum Arg {
    Required(String),
    Optional(String)
}

pub struct Args {
    program_name: Option<String>,
    command_line: Vec<String>,
    args: HashMap<String, Arg>,
    arg_names: Vec<String>,
    flags: Vec<String>,
    arg_count: usize,
    possible_flags: HashSet<String>,
    error_list: Vec<String>
}

impl Args {
    /// Creates a new `Self` populated with the command-line arguments.
    pub fn new() -> Self {
        let mut me = Self::from(env::args().skip(1).collect::<Vec<String>>());
        me.program_name = env::args().nth(0);

        me
    }

    /// Creates an empty `Self` without any parsed arguments.
    /// 
    /// Convenience method, used by `From` implementations that then populate
    /// the arguments.
    fn init_empty() -> Self {
        Self {
            program_name: None,
            command_line: Vec::new(),
            flags: Vec::new(),
            args: HashMap::new(),
            arg_names: Vec::new(),
            arg_count: 0,
            possible_flags: HashSet::new(),
            error_list: Vec::new()
        }
    }

    /// Specifies the name of a required field.
    /// 
    /// Required fields take an argument in the order that they are given on
    /// the command-line. If the number of required fields specified exceeds
    /// the number of command-line arguments given, it is considered an error
    /// by `check()`.
    /// 
    /// A name cannot be repeated by multiple fields.
    /// 
    /// Panics if a required field is specified after an optional field.
    pub fn required(&mut self, name: &str) -> &mut Self {
        let name = String::from(name);

        if self.args.contains_key(&name) {
            panic!("required argument '{name}' specified twice");
        }

        let optional_arg_count = self.args
                                    .values()
                                    .filter(|arg| matches!(arg, Arg::Optional(_)))
                                    .count();

        if optional_arg_count > 0 {
            panic!("required argument '{name}' specified after optional argument");
        }

        if self.arg_count >= self.command_line.len() {
            self.error_list.push(format!("required argument '{name}' not found"));
        }
        else {
            self.args.insert(name.clone(), Arg::Required(self.command_line[self.arg_count].clone()));
            self.arg_count += 1;
        }

        self.arg_names.push(format!("<{}>", name.clone()));
        
        self
    }


    /// Specifies the name of an optional field.
    /// 
    /// Optional fields take an argument in the order that they are given on
    /// the command-line, after any required fields. If the number of required
    /// fields specified exceeds the number of command-line arguments given,
    /// it is *not* considered an error by `check()`.
    /// 
    /// A name cannot be repeated by multiple fields.
    pub fn optional(&mut self, name: &str) -> &mut Self {
        let name = String::from(name);

        if self.args.contains_key(&String::from(&name)) {
            panic!("optional argument '{name}' specified twice");
        }

        if self.arg_count < self.command_line.len() {
            self.args.insert(name.clone(), Arg::Optional(self.command_line[self.arg_count].clone()));
            self.arg_count += 1;
        }

        self.arg_names.push(format!("[{}]", name.clone()));

        self
    }

    /// Specifies the name of an optional flag.
    /// 
    /// Flags are any command-line argument that begins with `-` or `+`.
    /// Panics if, when specifying the name of a flag, the qualifier is
    /// not given.
    /// 
    /// This method does not need to be called for a flag to be found by
    /// the `has_flag()` method. This method exists to build an example
    /// command-line for `Display`.
    /// 
    /// Panics if a flag name is repeated.
    pub fn flag(&mut self, name: &str) -> &mut Self {
        if self.possible_flags.contains(&String::from(name)) {
            panic!("flag '{}' specified twice", name);
        }

        self.possible_flags.insert(String::from(name));

        self
    }

    pub fn flag_required_or(&self, either: &str, or: &str) -> &mut Self {
        todo!()
    }

    /// Returns an error if any required fields are not found.
    /// 
    pub fn check(&self) -> ArgsResult<&Self> {
        if self.error_list.is_empty() {
            Ok(self)
        }
        else {
            Err(ArgsError::from(&self.error_list))
        }
    }

    /// Gives the value of a named argument, or `None` if it was not
    /// found.
    pub fn get_arg(&self, name: &str) -> Option<String> {
        let name = String::from(name);

        if self.args.contains_key(&name) {
            match &self.args[&name] {
                Arg::Required(arg) | Arg::Optional(arg) => Some(arg.clone()),
                _ => None
            }
        }
        else {
            None
        }
    }

    /// Indicates whether a specific flag was found on the command-line
    /// 
    /// The flag does not have to have been previously specified with the
    /// `flag()` method.
    pub fn has_flag(&self, name: &str) -> bool {
        self.flags.contains(&String::from(name))
    }
}

impl Debug for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Args");
        for name in &self.arg_names {
            let arg_type = match name.chars().nth(0).unwrap_or_default() {
                '<' => "required",
                '[' => "optional",
                _ => "arg"
            };
            f.field(arg_type, &name);
        }
        for flag in &self.possible_flags {
            f.field("flag", &flag);
        }
        f.finish()
    }
}

impl Display for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.program_name.clone().unwrap_or_default()));

        for name in &self.arg_names {
            f.write_fmt(format_args!("{} ", name));
        }

        for flag in &self.possible_flags {
            f.write_fmt(format_args!("[{}] ", flag));
        }

        Ok(())
    }
}

impl From<Vec<&str>> for Args {
    fn from(args: Vec<&str>) -> Self {
        let mut me = Self::init_empty();
        
        for arg in args {
            if ['-', '+'].contains(&arg.chars().nth(0).unwrap()) {
                me.flags.push(String::from(arg));
            }
            else {
                me.command_line.push(String::from(arg));
            }
        }
        
        me
    }
}

impl From<Vec<String>> for Args {
    fn from(args: Vec<String>) -> Self {
        let mut me = Self::init_empty();
        
        for arg in args {
            if ['-', '+'].contains(&arg.chars().nth(0).unwrap()) {
                me.flags.push(arg);
            }
            else {
                me.command_line.push(arg);
            }
        }
        
        me
    }
}


//---------------------------------------------------------------------------//


type ArgsResult<T> = Result<T, ArgsError>;

pub struct ArgsError {
    problems: Vec<String>
}

impl ArgsError {
    fn new(problem: String) -> Self {
        Self {
            problems: Vec::from(&[problem])
        }
    }

    pub fn get_problems(&self) -> &[String] {
        &self.problems
    }
}

impl Debug for ArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("ArgsError");
        for problem in &self.problems {
            f.field("error", &problem);
        }
        f.finish()
    }
}

impl Display for ArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArgsError - {} problems", self.problems.len())
    }
}

impl Error for ArgsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "ArgsError"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl From<&str> for ArgsError {
    fn from(value: &str) -> Self {
        Self::new(String::from(value))
    }
}

impl From<String> for ArgsError {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&[String]> for ArgsError {
    fn from(value: &[String]) -> Self {
        ArgsError {
            problems: Vec::from(value)
        }
    }
}

impl From<&Vec<String>> for ArgsError {
    fn from(value: &Vec<String>) -> Self {
        ArgsError {
            problems: value.clone()
        }
    }
}
