// 在这个练习中，我们有一个类型为 `u32` 的 `Vec` ，叫做 `numbers`，它的值范围在0到99之间。
// 我们希望在8个不同的线程中同时使用这组数字。每个线程将获取每隔第八个值的和，
// 并具有一个偏移量。
//
// 第一个线程（偏移量为0）将求和：0, 8, 16, …
// 第二个线程（偏移量为1）将求和：1, 9, 17, …
// 第三个线程（偏移量为2）将求和：2, 10, 18, …
// …
// 第八个线程（偏移量为7）将求和：7, 15, 23, …
//
// 每个线程应该拥有该数字向量的一个引用计数指针。但是 `Rc` 不是线程安全的，
// 因此我们需要使用 `Arc`。
//
// 不要纠结于线程的创建和合并，我们会在后续的线程练习中练习这些内容。

// Don't change the lines below.
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();  // [0,100)

    // TODO: Define `shared_numbers` by using `Arc`.
    // let shared_numbers = ???;
    let shared_numbers = Arc::new(numbers); // 使用 arc 来封装numbers，实现线程安全的共享引用

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        // TODO: Define `child_numbers` using `shared_numbers`.
        // let child_numbers = ???;
        let child_numbers = Arc::clone(&shared_numbers);  // 对 share_numbers 的共享引用

        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {offset} is {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}
