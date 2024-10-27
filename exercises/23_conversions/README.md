# 类型转换

Rust 提供了多种方法将一种类型的值转换为另一种类型。

最简单的类型转换形式是类型转换表达式，使用二元操作符 `as` 表示。例如，`println!("{}", 1 + 1.0);` 无法编译，因为 `1` 是整数，而 `1.0` 是浮点数。然而，`println!("{}", 1 as f32 + 1.0)` 应该可以编译。练习 [`using_as`](using_as.rs) 涵盖了这一内容。

Rust 还提供了可以实现类型转换的特性，这些特性在 [`convert`](https://doc.rust-lang.org/std/convert/index.html) 模块中可以找到。这些特性如下：

- `From` 和 `Into`，在 [`from_into`](from_into.rs) 中覆盖
- `TryFrom` 和 `TryInto`，在 [`try_from_into`](try_from_into.rs) 中覆盖
- `AsRef` 和 `AsMut`，在 [`as_ref_mut`](as_ref_mut.rs) 中覆盖  ，**将参数轻量地转换为（可变）引用**

此外，`std::str` 模块还提供了一个叫 [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) 的特性，用于通过字符串的 `parse` 方法将字符串转换为目标类型。如果针对某个类型 `Person` 正确实现了此特性，那么 `let p: Person = "Mark,20".parse().unwrap()` 应该可以编译并运行且不会出现 panic。

这些是***标准库内***将数据转换为所需类型的主要方法。

## Further information

These are not directly covered in the book, but the standard library has a great documentation for it.

- [conversions](https://doc.rust-lang.org/std/convert/index.html)
- [`FromStr` trait](https://doc.rust-lang.org/std/str/trait.FromStr.html)
