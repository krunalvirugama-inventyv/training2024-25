// (1)
use minigrep::Config;
use std::env;
use std::process;

// // cargo run -- searchstring example-filename.txt
// // cargo run -- test sample.txt
// cargo run -- the poem.txt
fn main() {
    let args: Vec<String> = env::args().collect();

    // let first = &args[0];
    // let query = &args[1];
    // let file_path = &args[2];

    // println!("First {first}");
    // println!("Searching for {query}");
    // println!("In file {file_path}");

    // let (query, file_path) = parse_config(&args);

    // let config = parse_config(&args);

    // let config = Config::new(&args);

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    // run(config);

    // if let Err(e) = run(config) {
    //     println!("Application error: {e}");
    //     process::exit(1);
    // }

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

// fn run(config : Config){

//     let contents =
//     fs::read_to_string(config.file_path).expect("Should have been able to read the file");
//     println!("With text:\n{contents}");

// }

// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(config.file_path)?;

//     println!("With text:\n{contents}");

//     Ok(())
// }
// struct Config {
//     query: String,
//     file_path: String,
// }

// impl Config {
// fn new(args: &[String]) -> Config {

//     if args.len() < 3 {
//         panic!("not enough arguments");
//     }

//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config { query, file_path }
// }

// fn build(args: &[String]) -> Result<Config , &'static str> {

//     if args.len() < 3 {
//         return Err("not enough arguments");
//     }

//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Ok(Config{ query , file_path})
// }
// }

// fn parse_config(args: &[String]) -> Config {
//     // let query = &args[1];
//     // let file_path = &args[2];

//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config { query, file_path }
// }
