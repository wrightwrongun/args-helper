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
    Optional(String),
    Flag(String)
}

pub struct Args {
    program_name: String,
    command_line: Vec<String>,
    args: HashMap<String, Arg>,
    flags: HashSet<String>,
    arg_count: usize,
    possible_flags: HashSet<String>,
    error_list: Vec<String>
}

impl Args {
    pub fn init() -> Self {
        Self::init_with(env::args().skip(1).collect())
    }

    pub fn init_with(args: Vec<String>) -> Self {
        let mut me = Self {
            program_name: String::new(),
            command_line: Vec::new(),
            flags: HashSet::new(),
            args: HashMap::new(),
            arg_count: 0,
            possible_flags: HashSet::new(),
            error_list: Vec::new()
        };
        
        for arg in args {
            if ['-', '+'].contains(&arg.chars().nth(0).unwrap()) {
                me.flags.insert(arg);
            }
            else {
                me.command_line.push(arg);
            }
        }
        
        me
    }

    pub fn require(&mut self, name: &str) -> &mut Self {
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
            self.args.insert(name, Arg::Required(self.command_line[self.arg_count].clone()));
            self.arg_count += 1;
        }
        
        self
    }

    pub fn optional(&mut self, name: &str) -> &mut Self {
        let name = String::from(name);

        if self.args.contains_key(&String::from(&name)) {
            panic!("optional argument '{name}' specified twice");
        }

        if self.arg_count < self.command_line.len() {
            self.args.insert(name, Arg::Optional(self.command_line[self.arg_count].clone()));
            self.arg_count += 1;
        }

        self
    }

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

    pub fn check(&self) -> ArgsResult<&Self> {
        if self.error_list.is_empty() {
            Ok(self)
        }
        else {
            Err(ArgsError::from(&self.error_list))
        }
    }

    pub fn get_arg(&self, name: &str) -> Option<String> {
        let name = String::from(name);

        println!("args[{}]", self.args.len());

        if self.args.contains_key(&name) {
            match &self.args[&name] {
                Arg::Required(arg) => Some(arg.clone()),
                Arg::Optional(arg) => Some(arg.clone()),
                _ => None
            }
        }
        else {
            None
        }
    }

    pub fn has_flag(&self, name: &str) -> bool {
        self.flags.contains(&String::from(name))
    }
}


//---------------------------------------------------------------------------//


type ArgsResult<T> = Result<T, ArgsError>;

pub struct ArgsError {
    problems: Vec<String>
}

impl ArgsError {
    pub fn new(problem: String) -> Self {
        Self::from(&[problem])        
    }

    pub fn from(problems: &[String]) -> Self {
        ArgsError {
            problems: Vec::from(problems)
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
