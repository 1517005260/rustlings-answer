// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> String {
    "blue".to_string()   // &str -> String  &str是对字符串的切片引用，String是可变的有自主权的字符串
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
