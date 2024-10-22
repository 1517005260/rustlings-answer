// 假设我们正在编写一个游戏，你可以用代币购买物品。所有物品的价格都是
// 5个代币，并且每次购买物品时都会有1个代币的处理费。
// 游戏中的玩家会输入他们想要购买的物品数量，而
// `total_cost` 函数将计算这些物品的总成本。由于
// 玩家输入的数量，我们将其作为字符串获取。他们可能输入了任何内容，不仅仅是数字！
//
// 目前，这个函数根本没有处理错误情况。我们想要做的是：如果我们用一个不是
// 数字的字符串调用 `total_cost` 函数，那么该函数将返回一个 `ParseIntError`。在这种情况下，我们希望
// 立即从我们的函数中返回那个错误，而不是尝试乘法和加法。
//
// 至少有两种实现方式都是正确的。但有一种方法要简洁得多！

use std::num::ParseIntError;

fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // TODO: Handle the error case as described above.
    let qty = item_quantity.parse::<i32>()?;  // ? 可以自动传播 Result-Err 或 Option-None
    // 即：如果解析成功，继续执行；如果解析失败，将错误返回，不继续执行当前函数

    Ok(qty * cost_per_item + processing_fee)  // 如果解析成功，则计算
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().kind(),
            &IntErrorKind::InvalidDigit,
        );
    }
}
