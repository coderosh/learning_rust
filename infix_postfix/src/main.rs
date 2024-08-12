mod eval_postfix;
mod infix_to_postfix;
mod operators;

use std::io::{self, Write};

use eval_postfix::eval_postfix;
use infix_to_postfix::infix_to_postfix;

fn main() {
    let mut input = String::new();

    let mut debug = false;

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let expr = input.trim();

        if expr.trim() == "" {
            continue;
        }

        if expr == "DEBUG" {
            debug = true;
            continue;
        }

        if expr == "exit" {
            println!("Exiting...");
            break;
        }

        let postfix = infix_to_postfix(&expr.to_owned());

        if debug {
            println!("Expr: {expr}\nPostfix: {:?}", postfix.join(","));
        }

        let result = eval_postfix(&postfix);

        match result {
            Some(value) => println!("{value}"),
            _ => println!("Invalid expression"),
        };
    }
}
