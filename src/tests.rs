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

#[cfg(test)]
mod tests {
    use crate::args::*;

    fn to_string_vec(array: Vec<&str>) -> Vec<String> {
        array.iter().map(|x| String::from(*x)).collect()
    }

    #[test]
    fn args_from_str() {
        let args = Args::from(vec!["abc", "def", "xyz"]);
        assert_eq!(format!("{:?}", args), "Args");
    }

    #[test]
    fn args_from_string() {
        let args = Args::from(to_string_vec(vec!["abc", "def", "xyz"]));
        assert_eq!(format!("{:?}", args), "Args");
    }

    #[test]
    fn args_get_program_name_none() {
        let mut args = Args::from(vec!["abc", "def"]);

        args.set_program_name(None);

        args.required("one")
            .optional("two");

        assert_eq!(args.get_program_name(), None);
    }

    #[test]
    fn args_get_program_name_some() {
        let mut args = Args::from(vec!["abc", "def"]);

        args.set_program_name(Some(String::from("yada-yada")));

        args.required("one")
            .optional("two");

        assert_eq!(args.get_program_name(), Some(String::from("yada-yada")));
    }

    #[test]
    fn args_set_program_name_none() {
        let mut args = Args::from(vec!["abc", "def"]);

        args.set_program_name(None);

        args.required("one")
            .optional("two");

        assert_eq!(format!("{}", args), "<one> [two] ");
    }

    #[test]
    fn args_set_program_name_some_no_path() {
        let mut args = Args::from(vec!["abc", "def"]);

        args.set_program_name(Some(String::from("hello-world")));

        args.required("one")
            .optional("two");

        assert_eq!(format!("{}", args), "hello-world <one> [two] ");
    }

    #[test]
    fn args_set_program_name_some_with_path() {
        let mut args = Args::from(vec!["abc", "def"]);

        args.set_program_name(Some(String::from("/x/y/z/hello-world")));

        args.required("one")
            .optional("two");

        assert_eq!(format!("{}", args), "hello-world <one> [two] ");
    }

    #[test]
    fn args_require() {
        let mut args = Args::from(vec!["abc", "def", "xyz"]);
        args.required("one");

        assert_eq!(args.get_arg("one"), Some(String::from("abc")));
    }

    #[test]
    fn args_require_multiple() {
        let mut args = Args::from(vec!["abc", "def", "xyz"]);
        args.required("one");
        args.required("two");
        args.required("three");

        assert_eq!(args.get_arg("one"), Some(String::from("abc")));
        assert_eq!(args.get_arg("two"), Some(String::from("def")));
        assert_eq!(args.get_arg("three"), Some(String::from("xyz")));
    }

    #[test]
    fn args_optional() {
        let mut args = Args::from(vec!["abc", "def", "xyz"]);
        args.optional("one");

        assert_eq!(args.get_arg("one"), Some(String::from("abc")));
    }

    #[test]
    fn args_optional_multiple() {
        let mut args = Args::from(vec!["abc", "def", "xyz"]);
        args.optional("one");
        args.optional("two");
        args.optional("three");

        assert_eq!(args.get_arg("one"), Some(String::from("abc")));
        assert_eq!(args.get_arg("two"), Some(String::from("def")));
        assert_eq!(args.get_arg("three"), Some(String::from("xyz")));
    }

    #[test]
    fn args_optional_after_required() {
        let mut args = Args::from(vec!["abc", "def", "xyz"]);
        args.required("one");
        args.optional("two");
        args.optional("three");

        assert_eq!(args.get_arg("one"), Some(String::from("abc")));
        assert_eq!(args.get_arg("two"), Some(String::from("def")));
        assert_eq!(args.get_arg("three"), Some(String::from("xyz")));
    }

    #[test]
    #[should_panic]
    fn args_duplicate_names() {
        let mut args = Args::from(vec!["abc", "def", "xyz"]);
        args.required("one");
        args.optional("two");
        args.optional("one");

        assert_eq!(args.get_arg("one"), Some(String::from("abc")));
        assert_eq!(args.get_arg("two"), Some(String::from("def")));
        assert_eq!(args.get_arg("three"), Some(String::from("xyz")));
    }

    #[test]
    #[should_panic]
    fn args_optional_before_required() {
        let mut args = Args::from(vec!["abc", "def", "xyz"]);
        args.required("one");
        args.optional("two");
        args.required("three");

        assert_eq!(args.get_arg("one"), Some(String::from("abc")));
        assert_eq!(args.get_arg("two"), Some(String::from("def")));
        assert_eq!(args.get_arg("three"), Some(String::from("xyz")));
    }

    #[test]
    fn args_chain_specifiers() {
        let mut args = Args::from(vec!["abc", "def", "xyz"]);
        args.required("one")
            .required("two")
            .optional("three")
            .optional("four")
            .flag("-a")
            .flag("-b");

        assert_eq!(args.get_arg("one"), Some(String::from("abc")));
        assert_eq!(args.get_arg("two"), Some(String::from("def")));
        assert_eq!(args.get_arg("three"), Some(String::from("xyz")));
    }

    #[test]
    fn args_get_arg_good_1() {
        let mut args = Args::from(vec!["abc"]);
        args.required("one");

        assert_eq!(args.get_arg("one"), Some(String::from("abc")));
    }

    #[test]
    fn args_get_arg_good_2() {
        let mut args = Args::from(vec!["abc", "def"]);
        args.required("one")
            .required("two");

        assert_eq!(args.get_arg("one"), Some(String::from("abc")));
        assert_eq!(args.get_arg("two"), Some(String::from("def")));
    }

    #[test]
    fn args_get_arg_good_3() {
        let mut args = Args::from(vec!["abc"]);
        args.optional("one");

        assert_eq!(args.get_arg("one"), Some(String::from("abc")));
    }

    #[test]
    fn args_get_arg_good_4() {
        let mut args = Args::from(vec!["abc", "def", "xyz"]);
        args.optional("one")
            .optional("two")
            .optional("three");

        assert_eq!(args.get_arg("one"), Some(String::from("abc")));
        assert_eq!(args.get_arg("two"), Some(String::from("def")));
        assert_eq!(args.get_arg("three"), Some(String::from("xyz")));
    }

    #[test]
    fn args_get_arg_good_5() {
        let mut args = Args::from(vec!["abc", "def", "xyz"]);
        args.required("one")
            .required("two")
            .optional("three");

        assert_eq!(args.get_arg("one"), Some(String::from("abc")));
        assert_eq!(args.get_arg("two"), Some(String::from("def")));
        assert_eq!(args.get_arg("three"), Some(String::from("xyz")));
    }

    #[test]
    fn args_get_arg_bad_1() {
        let mut args = Args::from(vec!["abc"]);
        args.required("one");

        assert!(args.get_arg("two").is_none());
    }

    #[test]
    fn args_get_arg_bad_2() {
        let mut args = Args::from(vec!["abc"]);
        args.optional("one");

        assert!(args.get_arg("two").is_none());
    }

    #[test]
    fn args_possible_flags() {
        let mut args = Args::from(vec!["abc", "def", "xyz"]);
        args.flag("-a");
        args.flag("+x");

        assert!(format!("{}", args).contains("[-a]"));
        assert!(format!("{}", args).contains("[+x]"));
    }

    #[test]
    fn args_has_flag_good() {
        let args = Args::from(vec!["abc", "def", "xyz", "-a"]);

        assert!(args.has_flag("-a"));
    }

    #[test]
    fn args_has_flag_bad_1() {
        let args = Args::from(vec!["abc", "def", "xyz", "-a"]);

        assert!(!args.has_flag("-b"));
    }

    #[test]
    fn args_has_flag_bad_2() {
        let args = Args::from(vec!["abc", "def", "xyz"]);

        assert!(!args.has_flag("-a"));
    }

    #[test]
    fn args_check_good_1() {
        let mut args = Args::from(vec!["abc", "def", "xyz"]);
        args.required("one")
            .required("two");

        assert!(args.check().is_ok());
    }

    #[test]
    fn args_check_good_2() {
        let mut args = Args::from(vec!["abc", "def", "xyz"]);
        args.required("one")
            .required("two")
            .optional("three")
            .optional("four");

        assert!(args.check().is_ok());
    }

    #[test]
    fn args_check_good_3() {
        let mut args = Args::from(vec!["abc", "def", "xyz"]);
        args.required("one")
            .required("two")
            .optional("three");

        assert!(args.check().is_ok());
    }

    #[test]
    fn args_check_bad() {
        let mut args = Args::from(vec!["abc"]);
        args.required("one")
            .required("two");

        assert!(args.check().is_err());
    }

    #[test]
    fn args_display_1() {
        let mut args = Args::from(vec!["abc"]);
        args.required("one")
            .optional("two")
            .flag("+x");

        assert_eq!(format!("{}", args), "<one> [two] [+x] ");
    }

    #[test]
    fn args_display_2() {
        let mut args = Args::from(vec!["abc", "def"]);
        args.required("one")
            .optional("two")
            .flag("+x");

        assert_eq!(format!("{}", args), "<one> [two] [+x] ");
    }

    #[test]
    fn args_display_no_program_name() {
        let mut args = Args::from(vec!["abc", "def"]);

        args.set_program_name(None);

        args.required("one")
            .optional("two");

        assert_eq!(format!("{}", args), "<one> [two] ");
    }

    #[test]
    fn args_display_with_program_name() {
        let mut args = Args::from(vec!["abc", "def"]);

        args.set_program_name(Some(String::from("hello-world")));

        args.required("one")
            .optional("two");

        assert_eq!(format!("{}", args), "hello-world <one> [two] ");
    }

    #[test]
    fn args_display_with_program_name_path() {
        let mut args = Args::from(vec!["abc", "def"]);

        args.set_program_name(Some(String::from("/x/y/z/hello-world")));

        args.required("one")
            .optional("two");

        assert_eq!(format!("{}", args), "hello-world <one> [two] ");
    }

    #[test]
    fn args_debug_1() {
        let mut args = Args::from(vec!["abc"]);
        args.required("one")
            .optional("two")
            .flag("+x");

        assert_eq!(format!("{:?}", args), "Args { required: \"<one>\", optional: \"[two]\", flag: \"+x\" }");
    }

    #[test]
    fn args_debug_2() {
        let mut args = Args::from(vec!["abc", "def"]);
        args.required("one")
            .optional("two")
            .flag("+x");

        assert_eq!(format!("{:?}", args), "Args { required: \"<one>\", optional: \"[two]\", flag: \"+x\" }");
    }

    #[test]
    fn args_debug_no_program_name() {
        let mut args = Args::from(vec!["abc", "def"]);

        args.set_program_name(None);

        args.required("one")
            .optional("two");

        assert_eq!(format!("{:?}", args), "Args { required: \"<one>\", optional: \"[two]\" }");
    }

    #[test]
    fn args_debug_with_program_name() {
        let mut args = Args::from(vec!["abc", "def"]);

        args.set_program_name(Some(String::from("hello-world")));

        args.required("one")
            .optional("two");

        assert_eq!(format!("{:?}", args), "Args { program_name: \"hello-world\", required: \"<one>\", optional: \"[two]\" }");
    }

    #[test]
    fn args_debug_with_program_name_path() {
        let mut args = Args::from(vec!["abc", "def"]);

        args.set_program_name(Some(String::from("/x/y/z/hello-world")));

        args.required("one")
            .optional("two");

        assert_eq!(format!("{:?}", args), "Args { program_name: \"hello-world\", required: \"<one>\", optional: \"[two]\" }");
    }

    #[test]
    fn argserror_new_str() {
        let error = ArgsError::from("hello, world!");

        assert!(!error.get_problems().is_empty());
    }

    #[test]
    fn argserror_from_string() {
        let error = ArgsError::from(String::from("hello, world!"));

        assert!(!error.get_problems().is_empty());
    }

    #[test]
    fn argserror_from_vec() {
        let error = ArgsError::from(&to_string_vec(vec!["one", "two", "three"]));

        assert!(!error.get_problems().is_empty());
    }

    #[test]
    fn argserror_get_problems_1() {
        let error = ArgsError::from("one");

        assert!(error.get_problems().len() == 1);
    }

    #[test]
    fn argserror_get_problems_2() {
        let error = ArgsError::from(&to_string_vec(vec!["one", "two", "three"]));

        assert!(error.get_problems().len() == 3);
    }

    #[test]
    fn argserror_debug_1() {
        let error = ArgsError::from("one");

        assert_eq!(format!("{:?}", error), "ArgsError { error: \"one\" }");
    }

    #[test]
    fn argserror_debug_2() {
        let error = ArgsError::from(&to_string_vec(vec!["one", "two", "three"]));

        assert_eq!(format!("{:?}", error), "ArgsError { error: \"one\", error: \"two\", error: \"three\" }");
    }

    #[test]
    fn argserror_display_1() {
        let error = ArgsError::from("one");

        assert_eq!(format!("{}", error), "ArgsError - 1 problems");
    }

    #[test]
    fn argserror_display_2() {
        let error = ArgsError::from(&to_string_vec(vec!["one", "two", "three"]));

        assert_eq!(format!("{}", error), "ArgsError - 3 problems");
    }
}