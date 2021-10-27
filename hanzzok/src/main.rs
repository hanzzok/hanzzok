use std::{fs::File, io::Write};

use libhanzzok::{
    codegen::compile_html_with_hint,
    core::{
        ast::Raw, code_plugin, emphasize_plugin, heading_plugin, icon_plugin, input_guide_plugin,
        list_plugin, math_plugin, quotation_plugin, youtube_plugin, Compiler,
    },
    escape,
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
        .with(youtube_plugin())
        .with(emphasize_plugin())
        .with(icon_plugin())
        .with(input_guide_plugin());

    let tokenizer = HanzzokTokenizer::from_source(source);
    let tokens: Vec<_> = tokenizer.collect();
    let nodes = parse_root(tokens, &compiler);
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
            max-width: calc(140ch + 4em);
            padding: 3em 1em;
            margin: auto;
            line-height: 1.75;
            font-size: 1.25em;

            display: flex;
            flex-direction: row;
            justify-content: center;
        }}
        td pre {{
            padding: 1em;
            margin: 0;
        }}
        td {{
            max-width: 70ch;
            height: auto;
            padding: 0;
        }}
        @media screen and (max-width: 140ch) {{
            td:first-of-type {{
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
            white-space: pre-wrap;
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
        .icon {{
            width: 1em;
            vertical-align: middle;
            display: inline;
        }}
        .table-of-contents {{
            font-size: 0.9em;
            line-height: 1.5;
        }}
        .table-of-contents ol {{
            margin-bottom: 0;
        }}
        kbd.system-text > samp {{
            font-weight: bold;
            font-family: sans-serif;
            display: inline-block;
        }}
        kbd.system-text > samp::first-letter {{
            text-decoration: underline;
        }}
        kbd.key {{
            background: #e2e2e2;
            padding: 0.1em 0.5em;
            border-radius: 0.2em;
        }}
    </style>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.13.18/dist/katex.min.css" integrity="sha384-zTROYFVGOfTw7JV7KUu8udsvW2fx4lWOsCEDqhBreBwlHI4ioVRtmIvEThzJHGET" crossorigin="anonymous">
    <script defer src="https://cdn.jsdelivr.net/npm/katex@0.13.18/dist/katex.min.js" integrity="sha384-GxNFqL3r9uRJQhR+47eDxuPoNE7yLftQM8LcxzgS4HT73tp970WS/wV5p8UzCOmb" crossorigin="anonymous"></script>
    <script defer src='https://unpkg.com/shiki'></script>
    <script>
        document.addEventListener("DOMContentLoaded", function() {{
            for (const element of document.getElementsByClassName('math-block')) {{
                katex.render(element.textContent, element, {{
                    throwOnError: false
                }});
            }}
            for (const element of document.getElementsByClassName('math-inline')) {{
                katex.render(element.textContent, element, {{
                    throwOnError: false,
                    display: false,
                }});
            }}
            shiki
                .getHighlighter({{
                    theme: 'nord',
                    langs: ['javascript']
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
    <table>
"#
    )?;
    let (context, compiled_nodes) = compile_html_with_hint(&nodes, &compiler);

    for (node, html_nodes) in compiled_nodes {
        let source = escape(
            &node
                .raw()
                .iter()
                .map(|t| t.text.clone())
                .collect::<String>(),
        )
        .to_string();
        if source.trim().is_empty() {
            continue;
        }
        write!(
            &mut f,
            r#"
        <tr>
            <td>
                <pre>{}</pre>
            </td>
            <td>
        "#,
            source
        )?;
        for html_node in html_nodes {
            html_node.write(&context, &mut f)?;
        }
        write!(
            &mut f,
            r#"
            </td>
        </tr>
        "#
        )?;
    }

    write!(
        &mut f,
        r#"
    </table>
</body>

</html>
"#
    )?;

    Ok(())
}
