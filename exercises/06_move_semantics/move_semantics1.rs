// TODO: Fix the compiler error in this function.
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> { // 将vec设为可变的
    let mut vec = vec;  // 这里同步修改

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);  // vec0 所有权转移。在传入新函数时，可以修改”是否可变“的属性
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
