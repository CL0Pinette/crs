use std::env;
use std::fs::File;
use std::path::{Display, Path};

mod lexer;
mod parser;
use lexer::lex;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => println!("Please provide a C file"),
        2 => {
            let file: &String = &args[1];
            let path: &Path = Path::new(file);
            let display: Display = path.display();

            let mut file: File = match File::open(&path) {
                Err(why) => panic!("couldn't open {}: {}", display, why),
                Ok(file) => file,
            };
            lex(&mut file);
        }
        _ => {
            println!("bruh")
        }
    }
}
