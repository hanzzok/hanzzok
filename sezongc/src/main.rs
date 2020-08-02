use libsezongc::Compiler;

fn main() {
    let source = include_str!("../test.sz");
    let mut compiler = Compiler::new();
    let source_file = compiler.create_virtual_source("../test.sz", source);
    println!("{}", source);
    let document = source_file.parse();
    for node in document {
        println!("{:?} at {}", node.value, node.span);
    }
}
