use libhanzzok::syntax::{parse_root, HanzzokTokenizer};

fn main() {
    let source = include_str!("../../README.hz");
    let tokenizer = HanzzokTokenizer::from_source(source);
    let tokens: Vec<_> = tokenizer.collect();
    /*
    for token in &tokens {
        println!("{}", token);
    }
    */
    let ast = parse_root(tokens);
    for node in ast {
        println!("{}", node);
    }
}
