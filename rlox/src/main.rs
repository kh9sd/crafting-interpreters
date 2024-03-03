use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    match args.len(){
        // case of one arg
        2 => 0,
        // no arg
        1 => 1,
        _ => {
            panic!("Usage: jlox [script]");
        }
    };
    // println!("Hello, world!");
}

fn run_file(path: String) {

}

fn run(source: String){
    
}
