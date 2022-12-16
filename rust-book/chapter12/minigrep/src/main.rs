use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // let query = &args[1];
    // let file_path = &args[2];
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing argumets: {err}");
        process::exit(1);
    });

    println!("Serach for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    };
    // dbg!(args);
}

#[warn(dead_code)]
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
