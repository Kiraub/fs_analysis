use std::error::Error;

mod analysis;

pub fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    analysis::count_symbols::print_follow_sets(args)
}

#[cfg(test)]
mod tests {

    //use super::*;

    #[test]
    fn test_output() {
        println!("Hello world, this test is a success!");
    }
}
