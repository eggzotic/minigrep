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
    pub fn build(
        mut args: impl Iterator<Item = String>, // args: &[String]
    ) -> Result<Self, &'static str> {
        // skip the first item (this binary), which is guaranteed(?) present
        args.next();
        // next item is either the case-insensitive switch, or the query-string
        let first_arg = match args.next() {
            Some(arg) => arg,
            None => return Err("insufficient args"),
        };
        let ignore_case_switch = first_arg == "-i";
        let query = if ignore_case_switch {
            match args.next() {
                Some(query) => query,
                None => return Err("no query found"),
            }
        } else {
            first_arg
        };
        let file_path = match args.next() {
            Some(path) => path,
            None => return Err("file path not provided"),
        };
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
