use std::fmt::format;

// The trait `AppendBar` has only one function which appends "Bar" to any object
// implementing this trait.
trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: Implement `AppendBar` for the type `String`.
    fn append_bar(mut self) -> Self {
        self.push_str("Bar");
        self  // 需要显式返回self，因为测试二会再调用一次
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {s}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), "FooBar");
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(String::from("").append_bar().append_bar(), "BarBar");
    }
}
