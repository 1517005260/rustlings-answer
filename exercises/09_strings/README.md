# Strings

Rust has two string types, a string slice (`&str`) and an owned string (`String`).
We're not going to dictate when you should use which one, but we'll show you how
to identify and create them, as well as use them.

## Further information

- [Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)


## &str 对比 String：

| 类型    | 所有权             | 可变性     | 存储位置                      | 性能                    | 转换                       | 最佳使用场景                     |
|:--------|:-------------------|:-----------|:-------------------------------|:-------------------------|:----------------------------|:---------------------------------|
| &str    | 借用（不拥有数据）  | 不可变     | 栈（通常指向已存在的数据）      | 快速，无需堆分配          | 可以转换为 `String`          | 编译时已知的固定字符串            |
| String  | 拥有（持有数据）    | 可变       | 堆                             | 需要堆分配，较慢          | 不需要转换                   | 动态字符串或需要修改的字符串      |


## 内存分配——&str

和[i32](../06_move_semantics/README.md/#4-所有权的例外copy类型)一致

```
+------------------+
|      栈          |
+------------------+
|    指向堆或      |
|  静态内存的指针  |
+------------------+
```

## 内存分配——String

```
+------------------+               +--------------------+
|      栈          |               |        堆          |
+------------------+               +--------------------+
|    指向堆内存的  |  ---------->  |    实际字符串数据   |
|      指针        |               +--------------------+
+------------------+
```