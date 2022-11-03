use langlib::interpreter::{Interpreter,Err};


fn main() -> Result<(), Err> {
    Interpreter::repl()?;

    Ok(())
}
