# Architecture

## Preprocessor

```rust
trait Preprocessor {
    fn preprocess(input: String) -> String;
}
```