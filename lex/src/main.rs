use lex::{Config, get_tokens};
use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments : {}",err);
        process::exit(1)
    });
    let tokens = get_tokens(config);
    println!("{:#?}",tokens);
}
