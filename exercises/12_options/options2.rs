fn main() {
    // You can optionally experiment here.
}

// 解决 单独unwrap 不安全的问题（如果取None值会引发异常）
#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if-let statement whose value is `Some`.
        // 使用if解决unwrap不安全的问题
        if let word = optional_target.unwrap()  // unwrap快速取出optional_target中的值
        {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: Make this a while-let statement. Remember that `Vec::pop()`
        // adds another layer of `Option`. You can do nested pattern matching
        // in if-let and while-let statements.
        // 使用 while 解决unwrap不安全的问题
        while let Some(Some(integer)) = optional_integers.pop() {  // optional_integers 本身是个装option类型的数组，而pop也返回option，所以须两层解包
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
