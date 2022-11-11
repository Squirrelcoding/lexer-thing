use std::path::Path;

use langlib::interpreter::Interpreter;

fn main() -> Result<(), langlib::interpreter::Err> {
    let interpreter = Interpreter::from_file(Path::new("test.lt"))?;

    interpreter.interpret()?;

    Ok(())
}
