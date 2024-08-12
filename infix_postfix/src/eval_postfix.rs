use crate::operators::get_op_precedence;

pub fn eval_postfix(postfix: &Vec<String>) -> Option<f32> {
    let mut stack = Vec::new();

    for char in postfix {
        let is_operator = get_op_precedence(char) != 0;

        if is_operator {
            let first = stack.pop()?;

            let second = stack.pop()?;

            let result = calculate(second, first, char);
            stack.push(result)
        } else if char != "" {
            match char.parse::<f32>() {
                Ok(value) => stack.push(value),
                _ => return None,
            }
        }
    }

    return Some(stack.pop()?);
}

fn calculate(a: f32, b: f32, op: &str) -> f32 {
    match op {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        _ => 0.0,
    }
}
