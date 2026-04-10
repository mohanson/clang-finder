# Clang Finder

A tiny rust crate that locates an available `clang` executable on the current system.

Primary use case: call it from `build.rs` to ensure native build steps use clang.

# Common usage

```toml
[dependencies]
cc = "1"
clang-finder = "0.1.0"
```

```rust
fn main() {
    cc::Build::new()
        .compiler(&clang_finder::find())
        .file("src/native/foo.c")
        .compile("foo");
}
```

# Search order

1. If the `CLANG` environment variable is set, its value is used as the clang executable path.
2. If `CLANG` is not set, the crate searches for clang executables in the system PATH, starting with `clang-30` down to `clang-17`. The first one found is returned.
