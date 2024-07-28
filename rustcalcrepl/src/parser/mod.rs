use crate::tokenizer::{Token, TokenType, Tokenizer};

#[derive(Debug)]
pub struct ExpressionNode {
    pub left: Box<Node>,
    pub right: Box<Node>,
    pub operator: String,
}

#[derive(Debug)]
pub enum Node {
    NumberLiteral(f64),
    Expression(ExpressionNode),
}

pub struct Parser {
    tokenizer: Tokenizer,
    current: Token,
}

impl Parser {
    pub fn new(src: String) -> Parser {
        let mut tokenizer = Tokenizer::new(src);
        let current = tokenizer.next();

        Parser { tokenizer, current }
    }
    pub fn eat(self: &mut Self, expect_type: TokenType) -> Token {
        if self.current.token_type != expect_type {
            panic!(
                "Expected type {:?} but got {:?}",
                expect_type, self.current.token_type
            )
        }

        return std::mem::replace(&mut self.current, self.tokenizer.next());
    }

    pub fn parse(self: &mut Self) -> Node {
        return self.parse_additive();
    }

    pub fn parse_additive(self: &mut Self) -> Node {
        let mut left = self.parse_multiplicative();

        while self.current.value == "-" || self.current.value == "+" {
            let operator = self.eat(TokenType::Operator);

            left = Node::Expression(ExpressionNode {
                left: Box::new(left),
                right: Box::new(self.parse_multiplicative()),
                operator: operator.value,
            })
        }
        return left;
    }

    pub fn parse_multiplicative(self: &mut Self) -> Node {
        let mut left = self.parse_number();

        while self.current.value == "*" || self.current.value == "/" {
            let operator = self.eat(TokenType::Operator);

            left = Node::Expression(ExpressionNode {
                left: Box::new(left),
                right: Box::new(self.parse_number()),
                operator: operator.value,
            })
        }

        return left;
    }

    pub fn parse_number(self: &mut Self) -> Node {
        let tok = self.eat(TokenType::Number);
        return Node::NumberLiteral(tok.value.parse().unwrap_or_default());
    }
}
