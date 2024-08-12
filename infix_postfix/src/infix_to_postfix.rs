use crate::operators::get_op_precedence;

pub fn infix_to_postfix(str: &String) -> Vec<String> {
    let mut postfix = Vec::new();
    let mut stack = Vec::new();

    let str = fix_multiply_before_bracket(str);

    let mut tokens: Vec<String> = Vec::new();
    str.split("").for_each(|char| {
        if tokens.is_empty() {
            tokens.push(char.to_string());
        }

        let last = tokens.last_mut().unwrap();

        let is_prev_num = last.parse::<f32>().is_ok();
        let is_cur_num = char.parse::<f32>().is_ok();

        if is_cur_num && is_prev_num {
            last.push_str(char);
        } else {
            tokens.push(char.to_string());
        }
    });

    tokens.iter().for_each(|char| {
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
                    postfix.push(stack.pop().unwrap().to_string());
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

                postfix.push(top_char.to_string());
            }
        } else {
            postfix.push(char.to_string());
        }
    });

    while !stack.is_empty() {
        postfix.push(stack.pop().unwrap().to_string());
    }

    return postfix;
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
