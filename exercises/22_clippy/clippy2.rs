fn main() {
    let mut res = 42;
    let option = Some(12);
    // TODO: Fix the Clippy lint.
    // clippy-warning: 不建议在for循环中引入Option
    // for x in option {
    //     res += x;
    // }
    if let Some(x) = option {
        res += x;
    }
    println!("{res}");
}
