// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`

// 相比上一题，这里我们需要再维护一个多线程并发下的变量：jobs_done

use std::{sync::Arc, thread, time::Duration};
use std::sync::Mutex;

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO: `Arc` isn't enough if you want a **mutable** shared state.
    // arc 仅是共享读，写还需另外加锁Mutex
    let status = Arc::new(Mutex::new (JobStatus { jobs_done: 0 }));

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // TODO: You must take an action before you update a shared value.
            // 使用 Mutex 的 lock 方法来安全地获取数据的可变引用
            // status_shared.lock() 返回一个 Result<JobStatus的可变引用,Err>
            let mut job_status = status_shared.lock().unwrap();
            job_status.jobs_done += 1;
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete.
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: Print the value of `JobStatus.jobs_done`.
    println!("Jobs done: {}", status.lock().unwrap().jobs_done);
}
