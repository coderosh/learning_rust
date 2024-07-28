pub mod interpreter;
pub mod parser;
pub mod tokenizer;

use linefeed::{Interface, ReadResult};

fn main() {
    repl();
}

fn repl() {
    if let Ok(reader) = Interface::new("simple-repl") {
        let _ = reader.set_prompt("> ");

        while let Ok(ReadResult::Input(input)) = reader.read_line() {
            if input == "" {
                continue;
            }

            let value = interpreter::eval(input);
            println!("{value}");
        }

        println!("Goodbye.");
    } else {
        println!("Something went wrong");
    }
}
