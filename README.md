# hanzzok

A small predictable markup language.

> To the big site while turning over a page.
> 
> 한 쪽씩 넘기며 한(큰) 쪽으로.

Goals:

- Make the language predictable
- Have a small ruleset
- Make documents store contents instead of styles
- Make components creation and adoption easy
- Make documents platform-agnostic
- Make tool integration easy and rich

Not a Goals:

- Complicated powerful typesetting system (like LaTeX) : We aren't making a paper book.
- Concise stackable emphasize syntax (like Markdown) : We wish to write more instead decorate.

Remaining Questions:

- How to integrate a complex syntax like a table or a sublist? -> Thinking of indentation and dedentation syntax, and a syntax just for the table.

## Atoms

### Block Constructor

#### Basic Form

``` hanzzok
"|" <name> <inline object>* <params>*
```

Example:

``` hanzzok
| youtube q2QfH1JqtkY { width: 100%, height: auto }
```

#### Shortened Form

``` hanzzok
<name-composed-of-special-characters> <inline object>* <params>*
```

Example:

``` hanzzok
# Heading { id: "another-anchor" }

// You can pass
   a multiline inline objects
   to the block constructor
   may the plugin warn about it.
```

#### Multiline Form 1

``` hanzzok
<name-composed-of-special-characters> <inline object>* <params>* <newline>
<raw text>
<name-composed-of-special-characters>
```

Example:

``` hanzzok
\``` hanzzok { highlight: [1, 3] }
\\``` hanzzok
Hello, world!
\\```
\```
```

#### Multiline Form 2

``` hanzzok
<name-composed-of-special-characters> <inline object>*
<name-composed-of-special-characters> <inline object>*
...
```

Example:

``` hanzzok
> "Was it a cat I saw?"
>
> Well, probably not.
```

### Inline Constructor

Must be used in the decorator chain.

``` hanzzok
"#" <space> <name> ( "(" <params>+ ")" )?
```

Example:

``` hanzzok
[#image("./images/test.png") .width(100%) .height(auto)]
```

### Decorator Chain

``` hanzzok
"[" <inline object> ("." <name> ("(" <params>+ ")")?)* "]"
```

Example:

``` hanzzok
[Hello .italic] [world! .bold .italic]
```
