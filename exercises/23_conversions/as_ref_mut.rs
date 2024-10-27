// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.

// 在这个示例中，需要使用 AsRef 和 AsMut trait 来实现泛型的引用转换。AsRef 允许轻量级的引用转换，而 AsMut 则允许对可变引用进行转换。

// Obtain the number of bytes (not characters) in the given argument.
// TODO: Add the `AsRef` trait appropriately as a trait bound.
// 计算字节数

// trait AsRef<T> {
//     fn as_ref(&self) -> &T;
// }
// asref 将 泛型参数 转换为 同一类型的引用
// 即在定义时的 T: AsRef<target> 会将传入的 T 转为 &target 类型
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument.
// TODO: Add the `AsRef` trait appropriately as a trait bound.
// 计算字符数
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Squares a number using `as_mut()`.
// TODO: Add the appropriate trait bound.
// 平方一个数字
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    // TODO: Implement the function body.
    let val = arg.as_mut();
    *val *= *val
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
