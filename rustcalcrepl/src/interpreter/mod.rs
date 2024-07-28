use crate::parser::{Node, Parser};

pub fn eval(src: String) -> f64 {
    let ast = Parser::new(src).parse();
    return evaluate(ast);
}

fn evaluate(ast: Node) -> f64 {
    return match ast {
        Node::Expression(exp) => {
            let operator = exp.operator.as_str();
            let left = evaluate(*exp.left);
            let right = evaluate(*exp.right);

            return match operator {
                "+" => left + right,
                "-" => left - right,
                "*" => left * right,
                "/" => left / right,
                _ => {
                    panic!("Invalid operator {}", operator);
                }
            };
        }
        Node::NumberLiteral(value) => value,
    };
}
