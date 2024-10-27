# Macros

Rust's macro system is very powerful, but also kind of difficult to wrap your
head around. We're not going to teach you how to write your own fully-featured
macros. Instead, we'll show you how to use and create them.

If you'd like to learn more about writing your own macros, the
[macrokata](https://github.com/tfpk/macrokata) project has a similar style
of exercises to Rustlings, but is all about learning to write Macros.

## Further information

- [Macros](https://doc.rust-lang.org/book/ch19-06-macros.html)
- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)


# 宏和函数的对比

| 特性           | 宏                                  | 函数                          |
|----------------|------------------------------------|-------------------------------|
| **编译期行为** | 编译期展开，生成代码               | 编译期生成机器代码            |
| **运行期行为** | 直接嵌入，无额外开销               | 运行时调用，有跳转开销        |
| **灵活性**     | 高，可处理各种输入                 | 低，参数类型和数量固定        |
| **类型检查**   | 展开后类型检查，错误难调试         | 编译期类型检查，错误易调试    |
| **可读性**     | 复杂，不易读和维护                 | 简单，易读和维护              |
| **适用场景**   | 生成重复或复杂的代码结构           | 处理确定性逻辑和操作          |


## 宏的优点在于：

- **灵活**：能够处理各种输入，生成复杂的代码结构，适用于重复性或多样性的代码生成需求。
- **高效**：在编译期展开，避免了运行时的额外开销，因此性能更高。

**宏的缺点**则在于：

- **复杂性**：编写宏语法较为复杂，尤其是使用 `macro_rules!` 等模式匹配语法时。
- **难以调试**：宏的错误信息通常在展开后的代码中出现，使得调试过程不如函数直观。

因此，宏适合那些需要在编译期生成高效代码的场景，但在可维护性和调试方面稍逊于函数。