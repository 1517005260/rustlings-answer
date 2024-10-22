// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// 让我们以函数的形式构建一个小机器。作为输入，我们将提供一个字符串列表和命令。 这些命令决定了要对字符串执行的操作。它可以是：
// - 将字符串转换为大写
// - 去除字符串的首尾空格
// - 向字符串末尾追加指定次数的“bar”
// 它的具体形式将是：
// - 输入将是一个由2元组组成的向量，第一个元素是字符串，第二个是命令。
// - 输出将是一个字符串向量。


enum Command {  // 匹配不同操作态
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use std::io::repeat;
    use super::Command;

    // TODO: Complete the function as described above.
    // pub fn transformer(input: ???) -> ??? { ??? }
    pub fn transformer(input:Vec<(String, Command)>) -> Vec<String>{
        input.into_iter()  // 先将 input 向量转换为迭代器，每次向后取一个
            .map(|(s, Command)| match Command {  // map = "对于" ， 即对于每个 (s,cmd) 元组
                Command::Uppercase => s.to_uppercase(),
                Command::Trim => s.trim().to_string(),
                Command::Append(n) => format!("{}{}", s, "bar".repeat(n)),
            }).collect()  // collect 方法 将收集 map 产生的元素
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    // use ???;
    use super::Command;
    use super::my_module::*;  // super 约等于 linux的 ..

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
