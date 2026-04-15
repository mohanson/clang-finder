# Clang Finder

A tiny rust crate that locates an available `clang` executable on the current system.

Primary use case: call it from `build.rs` to ensure native build steps use clang.

# Common usage

```toml
[dependencies]
cc = "1"
clang-finder = "0.1"
```

```rust
fn main() {
    cc::Build::new()
        .compiler(clang_finder::find())
        .file("src/native/foo.c")
        .compile("foo");
}
```

# Search order

1. If the `CLANG` environment variable is set, its value is used as the clang executable path.
2. If the `CLANG_VERSION` environment variable is set, the crate looks for an executable named `clang-<VERSION>` in the system PATH. If found, it is returned.
3. The crate searches for clang executables in the system PATH. It checks for `clang` only.
4. The crate searches for clang executables in the system PATH, starting with `clang-30` down to `clang-17`. The first one found is returned.
