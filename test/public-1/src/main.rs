use project::parser;
use std::io::Write;

fn main() {
    // let input = std::fs::read_to_string("input").expect("Input file seems to be missing");
    let input = "+ 1 2";

    let tokens = parser::lex(&input);
    let ast = parser::parse(tokens);

    let evaled: i32 = ast.eval();
    let evaled_str = evaled.to_string();

    let ast_str = ast.to_string();

    let f = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("output")
        .expect("unable to open output file");
    let mut f = std::io::BufWriter::new(f);

    write!(f, "{:?}\n", evaled_str).expect("Unable to write");
    write!(f, "{:?}", ast_str).expect("Unable to write");
    // assert_eq!(true, result);
}
