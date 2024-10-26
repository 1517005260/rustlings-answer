// 本练习探索了 `Cow` (Clone-On-Write) 智能指针。它可以封装并提供对借用数据的不可变访问，
// 并在需要变更或所有权时进行延迟克隆。`Cow` 设计为与通过 `Borrow` trait 实现的一般借用数据一起工作。

use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {  // 经典的Cow场景：当需要修改时（负数）为Owned，其余为borrowed
    for ind in 0..input.len() {
        let value = input[ind];
        if value < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[ind] = -value;
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() {
        // Clone occurs because `input` needs to be mutated.
        let vec = vec![-1, 0, 1];  // input需要修改，所以会发生克隆
        let mut input = Cow::from(&vec); // input初始状态为 Cow::Borrowed
        abs_all(&mut input);  // 发生了修改
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn reference_no_mutation() {
        // No clone occurs because `input` doesn't need to be mutated.
        let vec = vec![0, 1, 2];  // input全>=0，不需要修改，所以不会发生克隆
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        // TODO: Replace `todo!()` with `Cow::Owned(_)` or `Cow::Borrowed(_)`.
        assert!(matches!(input, Cow::Borrowed(_)));
    }

    #[test]
    fn owned_no_mutation() {
        // We can also pass `vec` without `&` so `Cow` owns it directly. In this
        // case, no mutation occurs (all numbers are already absolute) and thus
        // also no clone. But the result is still owned because it was never
        // borrowed or mutated.
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(vec);  // 不用引用生成Cow，即Cow直接拥有vec，即直接创建 Cow::Owned
        abs_all(&mut input);
        // TODO: Replace `todo!()` with `Cow::Owned(_)` or `Cow::Borrowed(_)`.
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn owned_mutation() {
        // Of course this is also the case if a mutation does occur (not all
        // numbers are absolute). In this case, the call to `to_mut()` in the
        // `abs_all` function returns a reference to the same data as before.
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO: Replace `todo!()` with `Cow::Owned(_)` or `Cow::Borrowed(_)`.
        assert!(matches!(input, Cow::Owned(_)));
    }
}
