# rustine luau lib

compiles luau code into bytecode

how 2 use:

```rust
rustine_luau::compile(source: &str) -> Result<Vec<u8>, CompileError>
```
