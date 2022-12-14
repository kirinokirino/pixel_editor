use std::collections::HashMap;
use std::env;

#[derive(Debug)]
pub struct Arguments {
    pub path: String,
    pub named: HashMap<String, String>,
    pub unnamed: Vec<String>,
}

impl Arguments {
    pub fn new() -> Self {
        let mut named: HashMap<String, String> = HashMap::new();
        let mut unnamed: Vec<String> = Vec::new();

        let mut iter = env::args();
        let path = iter.next().unwrap();
        let mut next_arg = iter.next();
        while next_arg.is_some() {
            let arg = unsafe { next_arg.unwrap_unchecked() };
            if arg.starts_with('-') {
                let key = arg.trim_start_matches('-');
                if key == "h" || key == "help" {
					print_help();
					std::process::exit(0);
                }
                let value = iter.next().expect("No value for optional key found!");
                named.insert(key.to_string(), value);
            } else {
                unnamed.push(arg);
            }

            next_arg = iter.next();
        }

        Self {
            path,
            named,
            unnamed,
        }
    }
}

fn print_help() {
	println!("{HELP}");
}

static HELP: &str = "Usage:
  pixel_editor --width 10 --height 10 <PATH>
  	open PATH as a .ppt image of width and size
  pixel_editor --help, pixel_editor -h
  	print this help message
";
