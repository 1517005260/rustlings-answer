// TODO: Fix the compiler error by adding one or two characters.
#[rustfmt::skip]  // 用于告诉 Rustfmt（Rust的自动格式化工具）跳过对特定代码块的格式化
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };  // 连续定义宏需要分号
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
