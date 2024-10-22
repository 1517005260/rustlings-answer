// Using catch-all error types like `Box<dyn Error>` isn't recommended for
// library code where callers might want to make decisions based on the error
// content instead of printing it out or propagating it further. Here, we define
// a custom error type to make it possible for callers to decide what to do next
// when our function returns an error.

// err5 在考虑自己写的err要兼容系统给的Box接口，本题在考虑自己实现err
use std::num::ParseIntError;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// A custom error type that we will be using in `PositiveNonzeroInteger::parse`.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {  // 自定义错误类，并分类为Creation和ParseInt。其中Creation又分为负数错和零错
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {  // 自定义错误处理方式（类函数）
    fn from_creation(err: CreationError) -> Self {
        Self::Creation(err)
    }

    // TODO: Add another error conversion function here.
    // fn from_parse_int(???) -> Self { ??? }
    fn from_parse_int(err: ParseIntError) -> Self {
        Self::ParseInt(err) // 将标准库中的ParseIntError转换为自己写的ParsePosNonzeroError::ParseInt
    }
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);  // 自定义结构体，即非零正整数

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {  // 生成非零正整数方法
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(Self(x as u64)),
        }
    }

    fn parse(s: &str) -> Result<Self, ParsePosNonzeroError> {  // 从字符串中解析一个非零正整数
        // TODO: change this to return an appropriate error instead of panicking
        // when `parse()` returns an error.
        // let x: i64 = s.parse().unwrap();  // unwrap遇到None会报错，这里尝试解决
        let x :i64 = s.parse().map_err(ParsePosNonzeroError::from_parse_int)?;  // 处理解析错误
        // 相当于，没问题时，执行到s.parse即可，如果有错误，映射错误到from_parse_int上解决
        Self::new(x).map_err(ParsePosNonzeroError::from_creation)  // 处理生成错误
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        assert!(matches!(
            PositiveNonzeroInteger::parse("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_)),
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            PositiveNonzeroInteger::parse("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative)),
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            PositiveNonzeroInteger::parse("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero)),
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42).unwrap();
        assert_eq!(x.0, 42);
        assert_eq!(PositiveNonzeroInteger::parse("42"), Ok(x));
    }
}
