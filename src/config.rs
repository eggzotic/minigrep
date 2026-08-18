use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    ignore_case_switch: bool,
    ignore_case_env: bool,
}

impl Config {
    // Case-insensitivity is enabled if either ENV or cmd-line switch is provided
    pub fn ignore_case(&self) -> bool {
        self.ignore_case_env || self.ignore_case_switch
    }

    // Build a Config instance from cmd-line args passed-in
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        let args_length = args.len();
        if args_length < 3 {
            return Err("not enough args");
        }
        let mut args_iter = args.iter();
        // skip the first item (this binary)
        args_iter.next();
        // next item is either the case-insensitive switch, or the query-string
        let first_arg = args_iter.next().unwrap();
        let ignore_case_switch = first_arg == "-i";
        // if case-insensitive switch is present, then the args need to be even longer
        if ignore_case_switch && args_length < 4 {
            return Err("not enough args");
        }
        let query = if ignore_case_switch {
            args_iter.next().unwrap()
        } else {
            first_arg
        }
        .clone();
        let file_path = args_iter.next().unwrap().clone();
        // check for case-insensitivity via the ENV as well
        let ignore_case_env = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case_switch,
            ignore_case_env,
        })
    }
}
