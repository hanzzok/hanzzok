use std::{fs::File, io::Write};

use libhanzzok::{
    codegen::compile_html,
    core::{
        code_plugin, heading_plugin, list_plugin, math_plugin, quotation_plugin, youtube_plugin,
        Compiler,
    },
    syntax::{parse_root, HanzzokTokenizer},
};

fn main() -> eyre::Result<()> {
    let source = include_str!("../../Showcase.hz");

    let compiler = Compiler::new()
        .with(heading_plugin())
        .with(quotation_plugin())
        .with(list_plugin())
        .with(math_plugin())
        .with(code_plugin())
        .with(youtube_plugin());

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
            max-width: 140ch;
            padding: 3em 1em;
            margin: auto;
            line-height: 1.75;
            font-size: 1.25em;
        }}
        main {{
            display: flex;
            flex-direction: row;
            justify-content: center;
        }}
        .left, .right {{
            max-width: 70ch;
        }}
        @media screen and (max-width: 140ch) {{
            .left {{
                display: none;
            }}
        }}
        h1,h2,h3,h4,h5,h6 {{
            margin: 0.5rem 0;
        }}
        p,ul,ol {{
            margin-bottom: 1em;
            color: #1d1d1d;
            font-family: sans-serif;
        }}
        pre {{
            font-size: inherit;
        }}
        blockquote {{
            margin: 0;
            border: 4px dashed #d2d2d2;
            border-left: 8px solid #c2c2c2;
            border-radius: 0 16px 16px 0;
            background: #e2e2e2;
            padding-left: 1em;
        }}
        .code-block > pre {{
            padding: 0.5rem;
        }}
    </style>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.13.18/dist/katex.min.css" integrity="sha384-zTROYFVGOfTw7JV7KUu8udsvW2fx4lWOsCEDqhBreBwlHI4ioVRtmIvEThzJHGET" crossorigin="anonymous">
    <script defer src="https://cdn.jsdelivr.net/npm/katex@0.13.18/dist/katex.min.js" integrity="sha384-GxNFqL3r9uRJQhR+47eDxuPoNE7yLftQM8LcxzgS4HT73tp970WS/wV5p8UzCOmb" crossorigin="anonymous"></script>
    <script src='https://unpkg.com/shiki'></script>
    <script>
        document.addEventListener("DOMContentLoaded", function() {{
            for (const element of document.getElementsByClassName('math-block')) {{
                katex.render(element.textContent, element, {{
                    throwOnError: false
                }});
            }}
            shiki
            .getHighlighter({{
                theme: 'nord'
            }})
            .then(highlighter => {{
                for (const element of document.getElementsByClassName('code-block')) {{
                    let code;
                    try {{
                        code = highlighter.codeToHtml(element.textContent, element.dataset['language']);
                    }} catch {{
                        code = highlighter.codeToHtml(element.textContent, 'text');
                    }}
                    element.innerHTML = code;
                    
                }}
            }});
        }});
    </script>
</head>

<body>
    <main>
        <pre class="left">
"#
    )?;
    write!(&mut f, "{}", source)?;
    write!(
        &mut f,
        r#"
        </pre>
        <div class="right">
"#
    )?;
    compile_html(&nodes, &compiler, &mut f)?;
    write!(
        &mut f,
        r#"
        </div>
    </main>
</body>

</html>
"#
    )?;

    Ok(())
}
