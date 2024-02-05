// Authors: Gilray, Gathara & Patel
// Project 0: Structs and Traits in Rust, Parsing/ASTs, S-expressions, Polish notation

// A parser that parses an S-expression language for Polish notation and converts it into an AST using the structs defined in the lib.rs file
// The language supports 4 artihmetic operations: +, -, *, ^ where arguments can be integers or other polish notation expressions
// Parentheses are optional for binary operators, but are mandator in other k-ary instances

// These are some examples of equivalent inputs:
//   + 1 25 => (+ 1 25)
//   - + 3 2 1 => (- (+ 3 2) 1)
//   (- 3 2 1) => (- (- 3 2) 1) Note that operations +, -, and * are all left associative
//   (^ 3 2 1) => (^ 3 (^ 2 1)) Note that the operation ^ is right associative
//   (- 2) => (- 0 2)
//   (+ 3) => 3

// Note that + and - support unary arguments, whereas * and ^ do not

//   Parse errors: return an ErrorExp struct
//   e.g.,
//   (* 4)     return ErrorExp, * and ^ cannot be used as unary operators
//   (^ 5)     ditto
//   ()        return ErrorExp, parens cannot go alone
//   (+)       return ErrorExp, operators must have 1 or more operands in all cases
//   x         return ErrorExp, only numeric 0-9 characters, whitespace, (, ), +, -, *, ^, are valid input characters

#[allow(dead_code)]
#[allow(unused_variables)]

pub mod parser {
    use core::panic;
    use regex::Regex;
    use std::error;

    pub trait Exp {
        fn print(&self);
        fn eval(&self) -> i32;
        fn to_string(&self) -> String;
        fn is_error(&self) -> bool;
    }

    #[derive(Clone)]
    pub struct PlusExp {
        pub lhs: std::rc::Rc<dyn Exp>,
        pub rhs: std::rc::Rc<dyn Exp>,
    }

    impl Exp for PlusExp {
        fn print(&self) {
            if self.lhs.is_error() || self.rhs.is_error() {
                print!("Error");
            } else {
                print!("(+ ");
                self.lhs.print();
                print!(" ");
                self.rhs.print();
                print!(")");
            }
        }

        fn eval(&self) -> i32 {
            if self.lhs.is_error() || self.rhs.is_error() {
                -1000000
            } else {
                self.lhs.eval() + self.rhs.eval()
            }
        }

        fn to_string(&self) -> String {
            if self.lhs.is_error() || self.rhs.is_error() {
                format!("error")
            } else {
                format!("(+ {} {})", self.lhs.to_string(), self.rhs.to_string())
            }
        }

        fn is_error(&self) -> bool {
            if self.lhs.is_error() || self.rhs.is_error() {
                true
            } else {
                false
            }
        }
    }

    #[derive(Clone)]
    pub struct ErrorExp;
    impl Exp for ErrorExp {
        fn print(&self) {
            println!("Error");
        }

        fn eval(&self) -> i32 {
            -1000000
        }

        fn to_string(&self) -> String {
            format!("error")
        }

        fn is_error(&self) -> bool {
            true
        }
    }

    // ---------------------------------------------------------------------------------------------------------------------

    //TODO: Implement MinusExp, MultExp, PowExp, LitExp and anything else you may need here
    /*
    Implement the other Expressions you need here
    HINT: MinusExp, MultExp, PowExp, LitExp, etc
    */

    #[derive(Clone)]
    pub struct MinusExp {
        pub lhs: std::rc::Rc<dyn Exp>,
        pub rhs: std::rc::Rc<dyn Exp>,
    }

    impl Exp for MinusExp {
        fn print(&self) {
            print!("(- "); //changed to -
            self.lhs.print();
            print!(" ");
            self.rhs.print();
            println!(")");
        }

        fn eval(&self) -> i32 {
            if self.lhs.is_error() || self.rhs.is_error() {
                -1000000
            } else {
                self.lhs.eval() - self.rhs.eval() //changed to -
            }
        }

        fn to_string(&self) -> String {
            if self.lhs.is_error() || self.rhs.is_error() {
                format!("error")
            } else {
                format!("(- {} {})", self.lhs.to_string(), self.rhs.to_string())
                // changed to -
            }
        }

        fn is_error(&self) -> bool {
            if self.lhs.is_error() || self.rhs.is_error() {
                true
            } else {
                false
            }
        }
    }

    #[derive(Clone)]
    pub struct MultExp {
        pub lhs: std::rc::Rc<dyn Exp>,
        pub rhs: std::rc::Rc<dyn Exp>,
    }

    impl Exp for MultExp {
        fn print(&self) {
            print!("(* "); //changed to *
            self.lhs.print();
            print!(" ");
            self.rhs.print();
            println!(")");
        }

        fn eval(&self) -> i32 {
            self.lhs.eval() * self.rhs.eval()
        }

        fn to_string(&self) -> String {
            if self.lhs.is_error() || self.rhs.is_error() {
                format!("error")
            } else {
                format!("(* {} {})", self.lhs.to_string(), self.rhs.to_string())
                // changed to *
            }
        }

        fn is_error(&self) -> bool {
            if self.lhs.is_error() || self.rhs.is_error() {
                true
            } else {
                false
            }
        }
    }

    #[derive(Clone)]
    pub struct PowExp {
        pub lhs: std::rc::Rc<dyn Exp>,
        pub rhs: std::rc::Rc<dyn Exp>,
    }

    impl Exp for PowExp {
        fn print(&self) {
            print!("(^ "); // changed to ^
            self.lhs.print();
            print!(" ");
            self.rhs.print();
            println!(")");
        }

        fn eval(&self) -> i32 {
            let base = self.lhs.eval();
            let exponent = self.rhs.eval();
            if exponent < 0 {
                println!("Error: negative exponent");
                return 1;
            }
            base.pow(exponent as u32)
        }

        fn to_string(&self) -> String {
            if self.lhs.is_error() || self.rhs.is_error() {
                format!("error")
            } else {
                format!("(^ {} {})", self.lhs.to_string(), self.rhs.to_string())
            }
        }

        fn is_error(&self) -> bool {
            self.lhs.is_error() || self.rhs.is_error()
        }
    }

    #[derive(Clone)]
    pub struct LitExp {
        pub n: i32,
    }

    impl Exp for LitExp {
        fn print(&self) {
            print!("{}", self.n);
        }

        fn eval(&self) -> i32 {
            self.n
        }

        fn to_string(&self) -> String {
            format!("{}", self.n)
        }

        fn is_error(&self) -> bool {
            false
        }
    }

    // ---------------------------------------------------------------------------------------------------------------------

    pub fn lex(exp: &str) -> Vec<&str> {
        /*

        */
        let mut toks: Vec<&str> = Vec::new(); // We initialize an empty vector to store tokens
        let mut last_index = 0;
        let mut chars = exp.char_indices().peekable(); // We create an iterator over char indices

        while let Some((i, c)) = chars.peek() {
            // We iterate over characters
            match c {
                '+' | '-' | '*' | '^' | '(' | ')' => {
                    // If we notice that there's a non-operator token, add it to the toks vector
                    if *i > last_index {
                        let tok = &exp[last_index..*i].trim();
                        if !tok.is_empty() {
                            toks.push(tok);
                        }
                    }
                    // Here we are getting a reference from chars.peek thus we need to dereference

                    // Using *i dereferences the i reference to get the actual usize value (the index)
                    toks.push(&exp[*i..*i + 1]);
                    last_index = *i + 1;
                }
                '0'..='9' => (), // Continue if the character is a digit, we want to collect digits until we form a full number
                _ => {
                    if *i > last_index {
                        let tok = &exp[last_index..*i].trim();
                        if !tok.is_empty() {
                            toks.push(tok);
                        }
                        last_index = *i + 1;
                    }
                }
            }
            chars.next();
        }

        // Add any remaining token at the end of the string such as ending ')'
        if last_index < exp.len() {
            let tok = &exp[last_index..exp.len()].trim();
            if !tok.is_empty() {
                toks.push(tok);
            }
        }

        toks
    }

    //TODO: Complete this function
    //     /*
    //         expect -> given a mutable vector with chars within, check if the token you are looking for is the one present at the top of the toks vector.

    //         given: x = ["a", "b", "c"]
    //         -> expect(x, "a") then mutate the toks vector to remove that value

    //         given: x = ["a", "b", "c"]
    //         -> expect(x, "b") then return a panic! (since "a" is at the front of the vector here)

    //     */
    pub fn expect<'a>(toks: &mut Vec<&'a str>, tok: &'a str) {
        if toks[0] == tok {
            toks.remove(0);
        } else {
            panic!("EXPECTED {} BUT GOT {}", tok, toks[0])
        }

        // let first_token_option = toks.first();
        // let matches_expected_token = match first_token_option {
        //     Some(first_token) => first_token == &tok, // AKA TRUE OR FALSE
        //     None => false, // FALSE IF EMPTY
        // };
        // if matches_expected_token {
        //     toks.remove(0);
        // } else {
        //     panic!("EXPECTED {} BUT GOT {}", tok, toks[0])
        // }
    }

    //TODO: Complete this function
    //     /*
    //         peek -> given a toks vector, "peek" into the element at the nth position of toks

    //         given: x = ["a", "b", "c"]
    //         -> peek(x, 0) would return "a"
    //         -> peek(x, 1) would return "b"
    //         -> peek(x, 5) would return ""
    //     */
    pub fn peek<'a>(toks: &'a Vec<&str>, n: usize) -> &'a str {
        if toks.len() > n {
            return &toks[n];
        } else {
            &""
        }
        // match toks.get(n) {
        //     Some(token) => token,
        //     None => &"",
        // }
    }

    pub fn parse(ts: Vec<&str>) -> std::rc::Rc<dyn Exp> {
        //TODO: Complete this function
        /*
            The lex function is responsible for breaking down the input expression into tokens.
            This is the first step of parsing where you identify the individual components of the expression
            The parse function is where you convert the tokenized input into an abstract syntax tree (AST).
            Think about how you can recursively build the tree by combining expressions based on the tokens
        */
        let mut toks = ts;

        pub fn parse_exp(toks: &mut Vec<&str>) -> std::rc::Rc<dyn Exp> {
            /*
                This function should recursively parse an expression based on the tokens
                Consider how each type of expression (PlusExp, MinusExp, etc.) should be parsed differently

            */

            // Consider the following example to parse (+ 1 2)
            let num = Regex::new(r"^\d+$").unwrap(); // Digits
            let ops = Regex::new(r"^(\+|-|\*|\^)$").unwrap(); // Operators

            let nexttok = toks[0];
            // + + 1 2 3
            // + 1 2 3
            match nexttok {
                "+" => {
                    expect(toks, nexttok); // This should remove the "+" from the front of toks
                    let arg1 = parse_exp(toks); // We recursively parse the first arg of "+"
                    let arg2 = parse_exp(toks); // and the same recursive parse of the second arg of "+""
                    return std::rc::Rc::new(PlusExp {
                        lhs: arg1,
                        rhs: arg2,
                    });
                }
                "-" => {
                    expect(toks, nexttok);
                    let arg1 = parse_exp(toks);
                    let arg2 = parse_exp(toks);
                    return std::rc::Rc::new(MinusExp {
                        lhs: arg1,
                        rhs: arg2,
                    });
                }
                "*" => {
                    expect(toks, nexttok);
                    let arg1 = parse_exp(toks);
                    let arg2 = parse_exp(toks);
                    return std::rc::Rc::new(MultExp {
                        lhs: arg1,
                        rhs: arg2,
                    });
                }
                // ^ 2 3 4
                "^" => {
                    expect(toks, nexttok);
                    let arg1 = parse_exp(toks);
                    let arg2 = parse_exp(toks);
                    return std::rc::Rc::new(PowExp {
                        lhs: arg1,
                        rhs: arg2,
                    });
                }

                // + 1 (2) wrong
                // + 1 (-2) fine
                // + 1 (+1) fine

                // + 1 ( + 1 (+ 1 2))
                "(" => {
                    expect(toks, nexttok);
                    let op = toks[0];
                    if ops.is_match(op) {
                        // The item right after a paren should be an operator
                        expect(toks, op);
                    } else {
                        return std::rc::Rc::new(ErrorExp); // Return an error if not
                    }
                    let mut next = peek(toks, 0); // This will not remove the item at the front of toks
                    let mut args: Vec<std::rc::Rc<dyn Exp>> = vec![]; // A vector to hold args within the parens
                    while next != ")" {
                        // Add the args until we see a right hand paren
                        let next_arg = parse_exp(toks);
                        args.push(next_arg);
                        next = peek(toks, 0);
                    }
                    expect(toks, ")");

                    if args.len() == 0 {
                        return std::rc::Rc::new(ErrorExp);
                    }

                    match op {
                        "+" => {
                            if args.len() == 1 {
                                // Addition allows for unary addition, thus we can use 0 for the left hand side.
                                // (+ 1) -> (+ 1 0)
                                return std::rc::Rc::new(PlusExp {
                                    lhs: std::rc::Rc::new(LitExp { n: 0 }),
                                    rhs: std::rc::Rc::clone(&args[0]),
                                });
                            }
                            // For binary or more arguments, we use the arg 0 as our left hand side number and arg 1 as our right hand arg.
                            // (+ 1 2) -> args[0] = 1 and args[1] = 2
                            let mut ast = std::rc::Rc::new(PlusExp {
                                lhs: std::rc::Rc::clone(&args[0]),
                                rhs: std::rc::Rc::clone(&args[1]),
                            });
                            for arg in &args[2..args.len()] {
                                ast = std::rc::Rc::new(PlusExp {
                                    lhs: ast, // Since we only allow for binary ASTs, we only allow for binary additions, thus our left hand side would be our previously calculated addition. In this case (+ (+ 1 2) 3) would return the AST you parsed for (+ 1 2)
                                    rhs: arg.to_owned(),
                                });
                            }
                            return ast;
                        }
                        "-" => {
                            if args.len() == 1 {
                                return std::rc::Rc::new(MinusExp {
                                    lhs: std::rc::Rc::new(LitExp { n: 0 }),
                                    rhs: std::rc::Rc::clone(&args[0]),
                                });
                            }

                            let mut ast = std::rc::Rc::new(MinusExp {
                                lhs: std::rc::Rc::clone(&args[0]),
                                rhs: std::rc::Rc::clone(&args[1]),
                            });
                            for arg in &args[2..args.len()] {
                                ast = std::rc::Rc::new(MinusExp {
                                    lhs: ast,
                                    rhs: arg.to_owned(),
                                });
                            }
                            return ast;
                        }
                        "*" => {
                            if args.len() == 1 {
                                return std::rc::Rc::new(MultExp {
                                    lhs: std::rc::Rc::new(LitExp { n: 0 }),
                                    rhs: std::rc::Rc::clone(&args[0]),
                                });
                            }

                            let mut ast = std::rc::Rc::new(MultExp {
                                lhs: std::rc::Rc::clone(&args[0]),
                                rhs: std::rc::Rc::clone(&args[1]),
                            });
                            for arg in &args[2..args.len()] {
                                ast = std::rc::Rc::new(MultExp {
                                    lhs: ast,
                                    rhs: arg.to_owned(),
                                });
                            }
                            return ast;
                        }
                      
                        "^" => {
                            if args.len() == 1 {
                                return std::rc::Rc::new(ErrorExp);
                            }
                            let mut ast = std::rc::Rc::new(PowExp {
                                lhs: std::rc::Rc::clone(&args[args.len() - 2]),
                                rhs: std::rc::Rc::clone(&args[args.len() - 1]),
                            });
                            for arg in &args[..args.len() - 2] {
                                ast = std::rc::Rc::new(PowExp {
                                    lhs: std::rc::Rc::clone(arg),
                                    rhs: ast,
                                });
                            }

                            return ast;
                        }
                        _ => {
                            expect(toks, nexttok);
                            return std::rc::Rc::new(LitExp {
                                n: nexttok.parse().unwrap(),
                            });
                        }
                    }
                }

                _ => {
                    // TODO: complete this match case
                    // Consider the possibility that you don't match on an op such as "+" above and you don't see an open paren

                    let val = toks[0];
                    expect(toks, nexttok);
                    return std::rc::Rc::new(LitExp {
                        n: val.parse().unwrap(),
                    });
                }
            }
            std::rc::Rc::new(ErrorExp)
        }

        let ast = parse_exp(&mut toks);
        if peek(&toks, 0) == "" {
            return ast;
        } else {
            return std::rc::Rc::new(ErrorExp);
        }
    }
}
