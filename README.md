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

- Complicated powerful typesetting system (like LaTeX)

Remaining Questions:

- How to integrate a complex syntax like a table or a sublist?

## Atoms

### Block Constructor

#### Basic Form

``` sezong
"|" <name> <inline object>* <params>*
```

Example:

``` sezong
| youtube q2QfH1JqtkY { width: 100%, height: auto }
```

#### Shortened Form

``` sezong
<name-composed-of-special-characters> <inline object>* <params>*
```

Example:

``` sezong
# Heading { id: "another-anchor" }
```

#### Multiline Form 1

``` sezong
<name-composed-of-special-characters> <inline object>* <params>* <newline>
<raw text>
<name-composed-of-special-characters>
```

Example:

``` sezong
\``` sezong { highlight: [1, 3] }
\\``` sezong
Hello, world!
\\```
\```
```

#### Multiline Form 2

``` sezong
<name-composed-of-special-characters> <inline object>*
<name-composed-of-special-characters> <inline object>*
...
```

Example:

``` sezong
> Was it a cat I saw?
>
> Well, probably not.
```

### Inline Constructor

Must be used in the decorator chain.

``` sezong
"#" <space> <name> ( "(" <params>+ ")" )?
```

Example:

``` sezong
[#image("./images/test.png" .width(100%) .height(auto))]
```

### Decorator Chain

``` sezong
"[" <inline object> ("." <name> ("(" <params>+ ")")?)* "]"
```

Example:

``` sezong
[Hello .italic] [world! .bold .italic]
```
