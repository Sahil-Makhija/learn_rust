use std::{env, error::Error, fs, process};

use minigrep::search;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::parse(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    // let _ = match run(config) { // No use for this
    //     Ok(()) => {} // No use for this either
    //     Err(err) => {
    //         println!("Application error: {err}");
    //         process::exit(1);
    //     }
    // };

    // Another way
    // let _ = run(config).unwrap_or_else(|err| println!("{err}"));

    // "If error occurs on doing `run(config)`, do this."
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn parse(args: Vec<String>) -> Result<Config, &'static str> {
        let args_len = args.len();
        if args_len < 3 {
            return Err("not enough arguments.");
        }

        let query = args[args_len - 2].clone(); // TODO: I do not want to use .clone()
        let file_path = args[args_len - 1].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_path)?;
    // println!("With text:\n{file_content}");

    for line in search(&config.query, &file_content, config.ignore_case) {
        println!("{line}");
    }

    Ok(())
}
