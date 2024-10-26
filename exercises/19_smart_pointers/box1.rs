// 在编译时，Rust 需要知道一个类型占用多少空间。
// 这在递归类型中会带来问题，因为一个值可能包含同类型的其他值作为其一部分。
// 为了解决这个问题，我们可以使用 `Box`——一种智能指针，用于在堆上存储数据，
// 这也让我们可以包装一个递归类型。
//
// 在这个练习中，我们要实现的是递归类型“cons 列表”，
// 这是一种在函数式编程语言中常见的数据结构。
// 每个 cons 列表的项包含两个元素：当前项的值和下一个项。
// 最后一项是一个名为 `Nil` 的值。

// TODO: Use a `Box` in the enum definition to make the code compile.
#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, Box<List>),  // 使用 Box 对 list进行封装，这样rust会知道list的大小并完成编译
    Nil,  // 空列表/列表终止
}

// TODO: Create an empty cons list.
fn create_empty_list() -> List {
    let list: List = List::Nil;  // 创建一个空列表
    list
}

// TODO: Create a non-empty cons list.
fn create_non_empty_list() -> List {
    // 列表元素为 Cons，首元素为列表元素值，次元素为指向下元素的指针
    let list: List = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    // 列表元素：[1] -> [2] -> Nil
    // 整体结构：
    //       Cons(1)
    //      /       \
    //    1          Cons(2)
    //              /       \
    //            2          Nil
    list
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}
