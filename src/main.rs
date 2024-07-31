use regex::Regex;

fn main() {
    let input = "3 + 5 * (10 - 4)";
    let tokens = tokenize(input);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    // You can print the AST or add more detailed checks here.
    println!("- {:?}", ast);
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
enum Token {
    Plus,
    Minus,
    Multiply,
    Divide,
    Number(i64),
    LParen,
    RParen,
}

fn tokenize(input: &str) -> Vec<Token> {
    let re = Regex::new(r"(\d+|\+|\-|\*|/|\(|\))").unwrap();
    let mut tokens = Vec::new();

    for cap in re.captures_iter(input) {
        let token = match &cap[0] {
            "+" => Token::Plus,
            "-" => Token::Minus,
            "*" => Token::Multiply,
            "/" => Token::Divide,
            "(" => Token::LParen,
            ")" => Token::RParen,
            num => Token::Number(num.parse().unwrap()),
        };
        tokens.push(token);
    }
    tokens
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum ASTNode {
    Number(i64),
    UnaryOp {
        op: Token,
        node: Box<ASTNode>,
    },
    BinOp {
        left: Box<ASTNode>,
        op: Token,
        right: Box<ASTNode>,
    },
}

#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn parse(&mut self) -> ASTNode {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> ASTNode {
        let mut node = self.parse_term();

        while self.pos < self.tokens.len() {
            match self.tokens[self.pos] {
                Token::Plus | Token::Minus => {
                    let op = self.tokens[self.pos].clone();
                    self.pos += 1;
                    let right = self.parse_term();
                    node = ASTNode::BinOp {
                        left: Box::new(node),
                        op,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        node
    }

    fn parse_term(&mut self) -> ASTNode {
        let mut node = self.parse_factor();
        println!("node {:#?}", node);

        while self.pos < self.tokens.len() {
            match self.tokens[self.pos] {
                Token::Multiply | Token::Divide => {
                    let op = self.tokens[self.pos].clone();
                    self.pos += 1;
                    let right = self.parse_factor();
                    node = ASTNode::BinOp {
                        left: Box::new(node),
                        op,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        println!("node after {:?}", node);

        panic!("here");

        // node
    }

    fn parse_factor(&mut self) -> ASTNode {
        dbg!(&self);
        match self.tokens[self.pos] {
            Token::Number(value) => {
                self.pos += 1;
                ASTNode::Number(value)
            }
            Token::LParen => {
                self.pos += 1;
                let node = self.parse_expression();
                if self.tokens[self.pos] == Token::RParen {
                    self.pos += 1;
                } else {
                    panic!("Expected closing parenthesis");
                }
                node
            }
            Token::Plus | Token::Minus => {
                let op = self.tokens[self.pos].clone();
                self.pos += 1;
                let node = self.parse_factor();
                ASTNode::UnaryOp {
                    op,
                    node: Box::new(node),
                }
            }
            _ => panic!("Unexpected token: {:?}", self.tokens[self.pos]),
        }
    }
}
