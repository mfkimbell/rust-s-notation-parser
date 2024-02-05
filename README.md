# rust-s-notation-parser
An s-notation parser in Rust. 


A parser that parses an S-expression language for Polish notation and converts it into an AST using the structs defined in the lib.rs file. The language supports 4 artihmetic operations: +, -, *, ^ where arguments can be integers or other polish notation expressions. Parentheses are optional for binary operators, but are mandator in other k-ary instances. "^" is right associative and + - * is left associative, so I built the parser accordingly. 

Ex 1:

`(+ 1 2 3 4)` = > `(+ (+ (+ 1 2) 3) 4)` => `10`

Ex 2: 

` (^ 3 2 1) => (^ 3 (^ 2 1))` => `9`

Ex 3:

`- + 3 2 1` => `(- (+ 3 2) 1)` => `4`

Ex 4:

 `(- 2)` => `(- 0 2)` => `-2`
