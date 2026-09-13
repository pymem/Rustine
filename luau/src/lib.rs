//! Minimal Luau bytecode compiler.
//!
//! Pipeline: `Luau source -> Rust wrapper -> compiled bytecode`.
//! This crate never touches process memory, the SDK, or execution.
//! It only compiles source so contributors can build execution later.
//!
//! Upstream: Luau by Roblox (https://github.com/luau-lang/luau, MIT).
//! Compiled here through `mlua` / `luau-src`
//! (https://github.com/mlua-rs/mlua, https://github.com/mlua-rs/luau-src-rs, MIT).

use std::fmt;

const MAX_SOURCE: usize = 1_000_000;

/// Compiler failure (syntax error, oversize input, internal error).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompileError(pub String);

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "compile error: {}", self.0)
    }
}

impl std::error::Error for CompileError {}

/// Compiles Luau source into Luau bytecode without running it.
///
/// Loading a chunk parses and compiles it; the returned function is
/// dumped to bytecode and never called.
pub fn compile(source: &str) -> Result<Vec<u8>, CompileError> {
    if source.is_empty() {
        return Err(CompileError("source is empty".into()));
    }
    if source.len() > MAX_SOURCE {
        return Err(CompileError("source too large (max 1MB)".into()));
    }
    let lua = mlua::Lua::new();
    let func = lua
        .load(source)
        .set_name("rustine")
        .into_function()
        .map_err(|e| CompileError(trim_error(&e.to_string())))?;
    func.dump()
        .map_err(|e| CompileError(trim_error(&e.to_string())))
}

fn trim_error(message: &str) -> String {
    let mut out: String = message.trim().chars().take(800).collect();
    if out.is_empty() {
        out = "unknown compiler failure".into();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiles_valid_source() {
        let bytecode = compile("local x = 1 return x").expect("valid source should compile");
        assert!(!bytecode.is_empty());
    }

    #[test]
    fn rejects_empty_source() {
        assert!(compile("").is_err());
    }

    #[test]
    fn rejects_syntax_error() {
        assert!(compile("local = = =").is_err());
    }
}
