// TODO: Fix the compiler error without taking the macro definition out of this
// module.
#[macro_use]  // 宏的作用域默认在模块内，需要加注解以在模块外部使用
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
