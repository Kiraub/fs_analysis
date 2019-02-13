
use std::env;
mod analysis;

fn main() {

    let args : Vec<String> = env::args().collect();
    if let Err(e) = analysis::count_symbols::run(args) {
        eprintln!("{}", e);
    }
}
