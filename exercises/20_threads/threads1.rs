// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.

use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    let mut handles = Vec::new();
    // 句柄handle：指向线程的特殊引用
    for i in 0..10 {
        let handle = thread::spawn(move || {  // thread::spawn 创建新线程
            // move || { ... } 创建一个临时程序块（闭包），新线程将拥有闭包里所有变量的所有权，避免多线程下的数据竞争
            // 闭包内是新线程需要干的事
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("Thread {i} done");
            start.elapsed().as_millis() //计算从 start 时间到当前时间的间隔，表示线程运行所花费的时间（以毫秒为单位），并返回 Result.T
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        // TODO: Collect the results of all threads into the `results` vector.
        // Use the `JoinHandle` struct which is returned by `thread::spawn`.
        //  JoinHandle 最常见的用途是通过调用 join 方法来等待线程完成执行
        // handle 是一个线程，调用join方法会阻塞至handle执行完毕，返回Result<T,Err>
        // unwrap用于解包Result.T
        let result = handle.join().unwrap();
        results.push(result);
    }

    if results.len() != 10 {
        panic!("Oh no! Some thread isn't done yet!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("Thread {i} took {result}ms");
    }
}
