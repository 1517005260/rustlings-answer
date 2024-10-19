#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();  // 大写

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);  // 后续还要用到data，所以用只读引用get

    string_uppercase(data); // 后续不用data了，可以不用引用，仅在传参时修改mut即可
}
