use crate::operators::get_op_precedence;

pub fn infix_to_postfix(str: &String) -> String {
    let mut postfix = Vec::new();
    let mut stack = Vec::new();

    let str = fix_multiply_before_bracket(str);

    str.split("").for_each(|char| {
        let is_operator = get_op_precedence(&char) != 0;

        if is_operator || char == "(" {
            loop {
                if stack.is_empty() {
                    stack.push(char);
                    break;
                }

                let stack_top_precedence = get_op_precedence(stack.last().unwrap());
                let current_char_precedence = get_op_precedence(&char);

                let both_are_ops = stack_top_precedence != 0 && current_char_precedence != 0;

                if both_are_ops && stack_top_precedence >= current_char_precedence {
                    postfix.push(stack.pop().unwrap());
                } else {
                    stack.push(char);
                    break;
                }
            }
        } else if char == ")" {
            loop {
                if stack.is_empty() {
                    break;
                }

                let top_char = stack.pop().unwrap();
                if top_char == "(" {
                    break;
                }

                postfix.push(top_char);
            }
        } else {
            postfix.push(char);
        }
    });

    while !stack.is_empty() {
        postfix.push(stack.pop().unwrap());
    }

    return postfix.join("");
}

fn fix_multiply_before_bracket(str: &String) -> String {
    let mut vec = Vec::new();
    let mut prev = "";

    let str = str.split_whitespace().collect::<String>();

    str.split("").for_each(|char| {
        if char == "(" && get_op_precedence(prev) == 0 {
            vec.push("*");
        }

        vec.push(char);

        prev = char;
    });

    return vec.join("");
}
