use std::path::Path;

use langlib::{
    interpreter::{Err, Interpreter}
};

fn main() -> Result<(), Err> {

    let interpreter = Interpreter::from_file(Path::new("test.lt"))?;

    interpreter.interpret()?;

    Ok(())
}
