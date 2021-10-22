use std::{fs::File, io::Write};

use libhanzzok::{
    codegen::compile_html,
    core::{heading_plugin, Compiler},
    syntax::{parse_root, HanzzokTokenizer},
};

fn main() -> eyre::Result<()> {
    let source = include_str!("../../README.hz");

    let compiler = Compiler::new().with(heading_plugin());

    let tokenizer = HanzzokTokenizer::from_source(source);
    let tokens: Vec<_> = tokenizer.collect();
    /*
    for token in &tokens {
        println!("{}", token);
    }
    */
    let nodes = parse_root(tokens, &compiler);
    for node in &nodes {
        println!("{}", node);
    }
    let mut f = File::create("./output.html")?;
    write!(
        &mut f,
        r#"
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
    <style>
        html {{
            max-width: 70ch;
            padding: 3em 1em;
            margin: auto;
            line-height: 1.75;
            font-size: 1.25em;
        }}
        h1,h2,h3,h4,h5,h6 {{
            margin: 0.5rem 0;
        }}
        p,ul,ol {{
            margin-bottom: 1em;
            color: #1d1d1d;
            font-family: sans-serif;
        }}
    </style>
</head>

<body>
    <main>
        <p>
            <div>
"#
    )?;
    compile_html(&nodes, &compiler, &mut f)?;
    write!(
        &mut f,
        r#"
            </div>
        </p>
    </main>
</body>

</html>
"#
    )?;

    Ok(())
}
