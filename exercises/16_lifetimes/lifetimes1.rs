// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?

// TODO: Fix the compiler error by updating the function signature.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }  // 修复之前，没有生命周期。当函数尝试返回两个之一的引用的时候会报错。
       // 因为编译器无法确定返回的引用的生命周期是否和 x 或 y 的生命周期一致，也无法判断它们是否可能在函数返回后已经无效
       // x,y 都是引用来的，在本函数的作用域内不清楚它们的来历，所以需要显式声明来历
       // 显然，单个引用时不需要生命周期
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abcd", "123"), "abcd");
        assert_eq!(longest("abc", "1234"), "1234");
    }
}
