# Smart Pointers

In Rust, smart pointers are variables that contain an address in memory and reference some other data, but they also have additional metadata and capabilities.
Smart pointers in Rust often own the data they point to, while references only borrow data.

## Further Information

- [Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [Using Box to Point to Data on the Heap](https://doc.rust-lang.org/book/ch15-01-box.html)
- [Rc\<T\>, the Reference Counted Smart Pointer](https://doc.rust-lang.org/book/ch15-04-rc.html)
- [Shared-State Concurrency](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
- [Cow Documentation](https://doc.rust-lang.org/std/borrow/enum.Cow.html)

# Rust 智能指针

Rust中的智能指针是一种封装了指针的结构体，提供了额外功能，如自动管理资源释放、引用计数等。

## 1. `Box<T>`
- **用途**：用于在堆上分配内存。
- **使用场景**：适用于编译时需决定大小的动态分配内存，或实现递归数据结构（如链表、树等）。
- **特性**：拥有数据的唯一所有权（单一所有权），超出作用域后自动释放内存。

### `Box` 的基本用法

| 用法            | 代码示例                                           | 说明                                                |
|-----------------|----------------------------------------------------|-----------------------------------------------------|
| **包装数据**    | `let boxed_value = Box::new(data);`               | 使用 `Box::new(data)` 将数据存储在堆上                |
| **访问数据**    | `*boxed_value`                                    | 解引用 `Box`，访问其内部数据                          |
| **递归结构**    | `Box::new(List::Cons(1, Box::new(List::Nil)))`     | 将 `Box` 用于嵌套递归结构                            |

## 2. `Rc<T>` (Reference Counted)
- **用途**：实现单线程环境中的多重所有权。
- **使用场景**：当需要多个所有者共享同一数据，且数据生命周期覆盖所有者生命周期时使用。
- **特性**：使用引用计数跟踪数据的拥有者，当所有引用离开作用域后，资源自动释放。`Rc<T>`不是线程安全的。

## 3. `Arc<T>` (Atomic Reference Counted)
- **用途**：与`Rc<T>`类似，但用于多线程环境。
- **使用场景**：在多线程中共享所有权，例如多个线程需要访问相同的堆数据。
- **特性**：基于原子操作，线程安全，但相较于`Rc<T>`性能稍低。

### Rc 与 Arc 对比

| 特性       | Rc                                                         | Arc                                                        |
|------------|------------------------------------------------------------|------------------------------------------------------------|
| **包装数据**    | 使用 `Rc::new(data)` 将数据包装成 `Rc` 类型               | 使用 `Arc::new(data)` 将数据包装成 `Arc` 类型               |
| **创建引用**    | 使用 `Rc::clone(&variable)` 创建共享引用，增加计数         | 使用 `Arc::clone(&variable)` 创建共享引用，增加计数         |
| **查询引用计数** | 使用 `Rc::strong_count(&variable)` 获取当前引用计数       | 使用 `Arc::strong_count(&variable)` 获取当前引用计数        |
| **删除引用**    | 通过 `drop` 删除引用，每删除一个引用计数减少，归零时释放    | 通过 `drop` 删除引用，每删除一个引用计数减少，归零时释放    |
| **线程安全**    | `Rc` 仅适用于单线程环境，不支持跨线程的并发访问           | `Arc` 是线程安全的，适合在多线程中共享，内部使用原子操作管理 |


## 4. `Cow<T>` (Clone-On-Write)
- **用途**：用于提供延迟克隆机制（clone-on-write），在需要修改数据时才会进行克隆操作。
- **使用场景**：适用于既可能需要借用数据又可能需要拥有数据的场景；当数据在多数情况下不需修改，但偶尔需要修改时，`Cow` 可以避免频繁克隆。
- **特性**：`Cow` 在读时保持借用（Cow::Borrowed），在写时进行克隆并转变为拥有（Cow::Owned）。通过延迟克隆，可以提升性能。

### `Cow` 的基本用法

| 用法              | 代码示例                                     | 说明                                                             |
|-------------------|----------------------------------------------|------------------------------------------------------------------|
| **借用数据**      | `let cow = Cow::Borrowed(&data);`            | 使用 `Cow::Borrowed` 包装借用数据                                 |
| **直接拥有数据**  | `let cow = Cow::Owned(data);`                | 使用 `Cow::Owned` 包装拥有数据                                   |
| **延迟克隆**      | `cow.to_mut()`                               | 使用 `to_mut()` 进行延迟克隆，仅在数据需修改时发生克隆             |
| **模式匹配**      | `match cow { Cow::Borrowed(_) => ..., Cow::Owned(_) => ... }` | 通过模式匹配确定数据是否为借用或拥有                             |