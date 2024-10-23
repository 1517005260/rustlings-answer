// Don't change this function.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {  //fn longest<'a> 声明此处有生命周期
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // TODO: Fix the compiler error by moving one line.

    let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(&string1, &string2);    // 悬空引用：str2的作用域仅限于{}，而str1存在于整个main；result = &str1 or &str2
    // }
    // 所以作出修改：调整作用域即可
    let string2 = String::from("xyz");
    let result = longest(&string1, &string2);
    println!("The longest string is '{result}'");
}
