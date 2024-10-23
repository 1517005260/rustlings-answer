// Tests are important to ensure that your code does what you think it should
// do.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]  // 测试模块的注解，只有在运行测试时，mod内的代码才能被编译运行
mod tests {
    // TODO: Import `is_even`. You can use a wildcard to import everything in
    // the outer module.
    use super::is_even;

    #[test]  // 测试函数的注解
    fn you_can_assert() {
        // TODO: Test the function `is_even` with some values.
        assert!(is_even(2));
        assert!(is_even(4));
    }
}
