
use std::env;

use fs_analysis;

fn main() {

    let args : Vec<String> = env::args().collect();
    match fs_analysis::run(args) {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e)
    }
}
