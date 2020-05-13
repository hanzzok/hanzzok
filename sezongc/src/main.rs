use libsezongc::{api::Spanned, Compiler, Parser, Tokenizer};

fn main() {
    use backtrace::Backtrace;
    let bt = Backtrace::new();

    let source = include_str!("../test.sz");
    let mut compiler = Compiler::new();
    let source_file = compiler.create_virtual_source("../test.sz", source);
    println!("{}", source);
    let tokenizer = Tokenizer::new(&source_file);
    let tokens: Vec<_> = tokenizer.collect();
    for token in &tokens {
        println!("{}", token);
    }
    let mut parser = Parser::from_tokens(&source_file, tokens.into_iter());
    let document = parser.parse();
    let nodes = match document {
        Ok(nodes) => {
            println!("Success with {} nodes", nodes.len());
            nodes
        }
        Err(error) => {
            println!("Fail with \n    {:?}\n    at {}", error, error.span());
            println!("{:?}", bt);
            return;
        }
    };
    for node in nodes {
        println!("{:?} at {}", node, node.span());
    }
}
