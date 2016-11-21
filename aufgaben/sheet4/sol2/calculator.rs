//! A simple command-line calculator
//!

/// The integer type we use internally to hold the numbers.
type Integer = i64;

/// Represents an operation our calculator can carry out.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Add,
    Subtract,
}

impl Op {
    /// The operation is applied to the given `lhs` and `rhs`. If an overflow
    /// occured during the operation, None is returned.
    fn apply(&self, lhs: Integer, rhs: Integer) -> Option<Integer> {
        match *self {
            Op::Add => lhs.checked_add(rhs),
            Op::Subtract => lhs.checked_sub(rhs),
        }
    }
}

/// Represents a (part of a) calculation.
#[derive(Debug, PartialEq)]
enum Expr {
    /// Just a literal number (no calculation needed)
    Literal(Integer),

    /// An operation with two operands
    Op {
        /// Kind of operation
        operator: Op,

        /// The Operands.
        ///
        /// The task said the operands should be saved as `Vec<Expr>`
        /// and it would be fine to hand in such a solution, but a vector
        /// is actually the wrong type here. We know that we always have
        /// exactly two operands, so a type that can hold any number of items
        /// is not correct.
        ///
        /// The correct type is a pair or an array with two elements. But this
        /// will lead to a problem: fields are saved inside of the type and
        /// therefore it's not possible to have a recursive definition. The
        /// only way to do it, is to introduce one layer of indirection. A
        /// vector is such a layer, because it saves its contents on the heap.
        ///
        /// A better choice is `Box<T>`. As a vector, it stores its content
        /// on the heap and also owns the content. But unlike the vector, a box
        /// always saves one element only.
        operands: Box<[Expr; 2]>,
    }
}

/// Stuff that can go wrong during parsing. Of course, it's desirable to have
/// even more information about an error, but this is not in the scope of this
/// task. Apart from that, good error reporting is hard. That's why so many
/// compiler (especially a few years ago) suck at it.
#[derive(Debug, Clone, Copy)]
enum ParseError {
    /// Empty input
    Empty,

    /// A token that was not expected at that position in the input
    UnexpectedToken(Token),

    /// More tokens were expected
    UnexpectedEof,

    /// There are unmatched parenthesis
    UnmatchedParens,

    /// More tokens in the input found, although none were expected
    UnexpectedAdditionalTokens,
}

impl Expr {
    /// This function parses a list of tokens and returns the corresponding
    /// expression tree, if the parse was successful. The grammar for our
    /// expressions is:
    ///
    /// ```
    /// expr    := ⟨operand⟩ [⟨op⟩ ⟨operand⟩]
    /// operand := ⟨num⟩ | "(" ⟨expr⟩ ")"
    /// op      := "+" | "-"
    /// num     := "0" | "1" | ... | "9"
    /// ```
    ///
    /// Parsing is difficult. Parsing mathematical infix notation like this can
    /// be done with the Shunting-yard algorithm [1]. But more general parsing
    /// algorithms to parse determinstic context-free grammars can be used,
    /// too. Recursive descent parsers are probably rather similar to the way
    /// humans think about a grammar.
    ///
    /// This implementation does not implement a concrete algorithm (as far as
    /// I know).
    ///
    /// [1]: https://en.wikipedia.org/wiki/Shunting-yard_algorithm
    fn parse(tokens: &[Token]) -> Result<Self, ParseError> {
        /// Parses an operand:
        ///
        /// ```
        /// operand := ⟨num⟩ | "(" ⟨expr⟩ ")"
        /// ```
        ///
        /// Note that it's usually nicer to build subslices of a slice instead
        /// of passing indices manually. But in this case it makes sense to
        /// work with "global" indices, because this function also returns an
        /// index into the slice.
        fn parse_operand(tokens: &[Token], start: usize)
            -> Result<(Expr, usize), ParseError>
        {
            // At this point, we expect an operand, so there need to be some
            // tokens left
            if start >= tokens.len() {
                return Err(ParseError::UnexpectedEof);
            }

            match tokens[start] {
                // The operand is just a literal number
                Token::Number(n) => Ok((Expr::Literal(n), start + 1)),

                // The operand is some operation
                Token::ParenOpen => {
                    match parse_expr(tokens, start + 1) {
                        Err(e) => Err(e),
                        Ok((expr, after_expr)) => {
                            // We expect the closing paren after the parsed
                            // expression
                            if tokens.get(after_expr) != Some(&Token::ParenClose) {
                                return Err(ParseError::UnmatchedParens);
                            }
                            Ok((expr, after_expr + 1))
                        }
                    }
                }

                // An operand starts either with a number or an opening paren
                tok => Err(ParseError::UnexpectedToken(tok)),
            }
        }

        /// Parses an expression:
        ///
        /// ```
        /// expr    := ⟨operand⟩ [⟨op⟩ ⟨operand⟩]
        /// ```
        ///
        fn parse_expr(tokens: &[Token], start: usize)
            -> Result<(Expr, usize), ParseError>
        {
            // First: parse the left hand side (lhs).
            let (lhs, after_lhs) = match parse_operand(tokens, start) {
                Err(e) => return Err(e),
                Ok(tuple) => tuple,
            };

            // If the lhs consumed all of our tokens, the LHS is the only
            // expression -> return it. Otherwise we expect an operator.
            let op = match tokens.get(after_lhs) {
                Some(&Token::Plus) => Op::Add,
                Some(&Token::Minus) => Op::Subtract,
                _ => return Ok((lhs, after_lhs)),
            };

            // Parse the rhs.
            let (rhs, after_rhs) = match parse_operand(tokens, after_lhs + 1) {
                Err(e) => return Err(e),
                Ok(tuple) => tuple,
            };

            Ok((
                Expr::Op {
                    operator: op,
                    operands: Box::new([lhs, rhs]),
                },
                after_rhs,
            ))
        }


        // Check for special case: no tokens.
        if tokens.is_empty() {
            return Err(ParseError::Empty);
        }

        // Start parsing the token stream as ⟨expr⟩
        let (expr, after_expr) = match parse_expr(&tokens, 0){
            Err(e) => return Err(e),
            Ok(t) => t,
        };

        // Check if tokens are left (shouldn't happen) and return an error
        // if that is the case.
        if after_expr < tokens.len() {
            return Err(ParseError::UnexpectedAdditionalTokens);
        }

        Ok(expr)
    }

    /// Evaluates the expression tree to calculate the final result. If an
    /// overflow occured, None is returned.
    fn evaluate(&self) -> Option<Integer> {
        match *self {
            Expr::Literal(value) => Some(value),
            Expr::Op { operator, ref operands } => {
                let lhs = match operands[0].evaluate() {
                    None => return None,
                    Some(lhs) => lhs,
                };

                let rhs = match operands[1].evaluate() {
                    None => return None,
                    Some(rhs) => rhs,
                };

                operator.apply(lhs, rhs)
            }
        }
    }
}

/// A token in the input stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Plus,
    Minus,
    ParenOpen,
    ParenClose,
    Number(Integer),
}

#[derive(Debug, Clone, Copy)]
enum LexError {
    InvalidChar(char),
    Overflow,
}

/// Tokenizes the string and returns a list of tokens, if the input is valid.
fn tokenize(input: &str) -> Result<Vec<Token>, LexError> {
    let mut out = Vec::new();
    let mut current_number = String::new();

    for c in input.chars() {
        // Check if we were reading a number and reached the end of it
        if !current_number.is_empty() && !(c >= '0' && c <= '9') {
            let num: Integer = match current_number.parse() {
                // The string contains valid digits only, so "overflow" is the
                // only reason `parse()` would error.
                Err(_) => return Err(LexError::Overflow),
                Ok(v) => v,
            };
            out.push(Token::Number(num));
            current_number.clear();
        }

        let token = match c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '(' => Token::ParenOpen,
            ')' => Token::ParenClose,
            c @ '0' ... '9' => {
                // We do not yet generate a token, but only push the char on
                // the number buffer
                current_number.push(c);
                continue;
            },
            c if c.is_whitespace() => continue,
            c => return Err(LexError::InvalidChar(c)),
        };
        out.push(token);
    }

    // If the last token is a number, we still have to push it
    if !current_number.is_empty() {
        let num: Integer = match current_number.parse() {
            // The string contains valid digits only, so "overflow" is the
            // only reason `parse()` would error.
            Err(_) => return Err(LexError::Overflow),
            Ok(v) => v,
        };
        out.push(Token::Number(num));
    }

    Ok(out)
}

fn main() {
    loop {
        // Read input from the user and just do nothing when the input is empty
        let input = read_string();
        if input.is_empty() {
            continue;
        }

        // Try to tokenize the input. If it fails, print error message and
        // start anew.
        let tokens = match tokenize(&input) {
            Err(e) => {
                println!("{:?}", e);
                continue;
            }
            Ok(tokens) => tokens,
        };

        // Debug output
        // println!("{:?}", tokens);

        // Try to parse the tokenstream into an expression. If it fails,
        // print the error and start anew.
        let expr = match Expr::parse(&tokens) {
            Err(e) => {
                println!("error: {:?}", e);
                continue;
            }
            Ok(expr) => expr,
        };

        // Debug output
        // println!("{:?}", expr);

        // Evaluate the expression and print the result
        match expr.evaluate() {
            None => println!("Overflow occured!"),
            Some(res) => println!("{}", res),
        }
    }
}


/// Reads a string from the user (with a nice prompt).
fn read_string() -> String {
    use std::io::Write;

    // Print prompt
    print!("calc > ");
    std::io::stdout().flush().unwrap();

    // Read line
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("something went horribly wrong...");

    // Discard trailing newline
    let new_len = buffer.trim_right().len();
    buffer.truncate(new_len);

    buffer
}


#[test]
fn parser_valid() {
    use Token::*;

    // "3"
    assert_eq!(
        Expr::parse(&[Number(3)]).unwrap(),
        Expr::Literal(3)
    );

    // "(3)"
    assert_eq!(
        Expr::parse(&[ParenOpen, Number(3), ParenClose]).unwrap(),
        Expr::Literal(3)
    );

    // "3 + 4"
    assert_eq!(
        Expr::parse(&[Number(3), Plus, Number(4)]).unwrap(),
        Expr::Op {
            operator: Op::Add,
            operands: Box::new([
                Expr::Literal(3),
                Expr::Literal(4),
            ]),
        }
    );

    // "(3 + 4)"
    assert_eq!(
        Expr::parse(&[ParenOpen, Number(3), Plus, Number(4), ParenClose]).unwrap(),
        Expr::Op {
            operator: Op::Add,
            operands: Box::new([
                Expr::Literal(3),
                Expr::Literal(4),
            ]),
        }
    );

    // "(3) + 4"
    assert_eq!(
        Expr::parse(&[ParenOpen, Number(3), ParenClose, Plus, Number(4)]).unwrap(),
        Expr::Op {
            operator: Op::Add,
            operands: Box::new([
                Expr::Literal(3),
                Expr::Literal(4),
            ]),
        }
    );

    // "(3 - 7) + 4"
    assert_eq!(
        Expr::parse(&[
            ParenOpen, Number(3), Minus, Number(7), ParenClose, Plus, Number(4)
        ]).unwrap(),
        Expr::Op {
            operator: Op::Add,
            operands: Box::new([
                Expr::Op {
                    operator: Op::Subtract,
                    operands: Box::new([
                        Expr::Literal(3),
                        Expr::Literal(7),
                    ]),
                },
                Expr::Literal(4),
            ]),
        }
    );
}

#[test]
fn parser_invalid() {
    use Token::*;

    // ""
    assert!(Expr::parse(&[]).is_err());

    // "-"
    assert!(Expr::parse(&[Minus]).is_err());

    // "1 +"
    assert!(Expr::parse(&[Number(1), Plus]).is_err());

    // "1 + 2 + 3"
    assert!(Expr::parse(&[Number(1), Plus, Number(2), Plus, Number(3)]).is_err());

    // "(1"
    assert!(Expr::parse(&[ParenOpen, Number(1)]).is_err());

    // "1)"
    assert!(Expr::parse(&[Number(1), ParenClose]).is_err());
}
