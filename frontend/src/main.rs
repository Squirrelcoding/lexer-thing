use std::path::Path;

use langlib::{interpreter::Interpreter};

fn main() -> Result<(), langlib::interpreter::Err> {
    let start = std::time::SystemTime::now();
    Interpreter::from_file(Path::new("test.lt"))?.interpret()?;
    println!("{}", start.elapsed().unwrap().as_nanos());
    Ok(())
}
