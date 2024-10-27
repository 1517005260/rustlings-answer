# Clippy

The Clippy tool is a collection of lints to analyze your code so you can catch common mistakes and improve your Rust code.

If you used the installation script for Rustlings, Clippy should be already installed.
If not you can install it manually via `rustup component add clippy`.

## Further information

- [GitHub Repository](https://github.com/rust-lang/rust-clippy).


## Clippy是什么

Clippy 是 Rust 编程语言的一个代码分析工具，用来帮助开发者查找和改进代码中的潜在问题或代码质量问题。它是一个 Rust 编译器插件，提供了一组静态代码分析规则，可以对 Rust 代码进行更严格的检查。Clippy 尤其适合在开发过程中作为代码风格和优化的辅助工具，它能给出很多建议，帮助代码更符合 Rust 的最佳实践。

想要使用clippy，直接在命令行中`cargo clippy`即可。