use std::env;
use std::io::{self, BufRead};
mod scanner;
mod parser;
mod interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    match args.len(){
        // case of one arg
        2 => run_file(&args[1]),
        // no arg
        1 => run_prompt(),
        _ => {
            panic!("Usage: jlox [script]");
        }
    };
    // println!("Hello, world!");
}

fn run_prompt() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        run(&line.expect("Fuck bro idk stdin failed???"));
    }
}

fn run_file(path: &String) {
    run(&std::fs::read_to_string(path).expect("Could not read from file"));
}

fn run(source: &String){
    let token_list = scanner::scan_tokens(&source);
    let expression_tree = parser::parse(&token_list);
    println!("{:?}", interpreter::stringify_valuetype(&interpreter::evaluate(&expression_tree)));
}
