# Sezong

```bnf
InlineConstructor :=
  "!["
    DecoratorFunction
    ("." DecoratorFunction)*
  "]"
Decorator :=
  "["
    Text?
    ("." DecoratorFunction)*
  "]"
DecoratorFunction := Ident ( "(" RAW ")" )?
BlockConstructor := "|" Ident BlockConstructorBody
SpecialBlockConstructor := ASCII_SPECIAL+ BlockConstructorBody
BlockConstructorBody :=
  Text
  (
    "()" |
    (
      "("
        Ident "=" Expr ("," Ident "=" Expr)* ","?
      ")"
    )
  )?
  (
    "{"{n}
    RAW
    "}"{n}
  )?
```