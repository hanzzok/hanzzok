use libhanzzok::syntax::HanzzokTokenizer;

fn main() {
    let source = include_str!("../../README.hz");
    let tokenizer = HanzzokTokenizer::from_source(source);
    for token in tokenizer {
        println!("{}", token);
    }
}
