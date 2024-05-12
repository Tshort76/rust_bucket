# Language Design
## Motivation
- safe, easy to use parallel computing paradigms, fast
C and C++ have hundreds of rules for avoiding undefined behavior  — mostly common sense items like don’t access memory you shouldn’t, don’t let arithmetic operations overflow, don’t divide by zero — but the **compiler does not enforce these rules**.  Indeed, the following program compiles without errors or warnings even though it could be used (e.g. 1988 Morris Worm) to access other programs on the call stack
```c
int main(int argc, char **argv) {
	unsigned long a[1];
	a[3] = 0x7ffff7b36cebUL;
	return 0;
}
```

With Rust, if your program passes the compiler’s checks, it is free of undefined behavior: dangling pointers, double-frees, and null pointer dereferences are caught at compile time, while array references are secured with a mix of compile-time and run-time checks.

# Gotchas
- `!` marks a macro invocation, not a function call
- Rust does not have exceptions: all errors are handled using either a `Result` or a panic
# Tooling

## Cargo
Cargo is Rust’s compilation manager, package manager, and general-purpose tool. You can use Cargo to start a new project, build and run your program, and manage any external libraries your code depends on. 

### Usage
- `cargo new hello --bin` – create a new (binary executable) project
- `cargo build` - compile your code
  - add `--release` for optimization (slower compilation, faster execution)
- `cargo run` – compile and run the resulting executable
- `cargo clean` – remove compiled files and the `target` folder

# Glossary
- **crate** : A packaged library or executable
- **rustc** : Rust compiler, often 
- **rustdoc** : Rust documentation tool. Doc comments, denoted by `///`, support Markdown and if present are automatically turned into useful html documentation for your code
