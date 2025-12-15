// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::sync::mpsc;
// 修复1：删除未使用的Arc导入
// use std::sync::Arc; 
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> Vec<thread::JoinHandle<()>> {
    let tx1 = tx.clone();
    let tx2 = tx;

    let first_half = q.first_half;
    let second_half = q.second_half;

    // 修复2：捕获线程句柄，用于主线程等待子线程结束
    let handle1 = thread::spawn(move || {
        for val in first_half {
            println!("sending {:?}", val);
            // 修复3：添加发送失败的错误提示（增强鲁棒性）
            if let Err(e) = tx1.send(val) {
                eprintln!("发送失败: {}", e);
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    let handle2 = thread::spawn(move || {
        for val in second_half {
            println!("sending {:?}", val);
            if let Err(e) = tx2.send(val) {
                eprintln!("发送失败: {}", e);
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 返回线程句柄，让主线程等待
    vec![handle1, handle2]
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    // 修复4：接收线程句柄
    let handles = send_tx(queue, tx);

    let mut total_received: u32 = 0;
    // 核心修复：先通过into_iter()转为迭代器，再调用take()
    for received in rx.into_iter().take(queue_length as usize) {
        println!("Got: {}", received);
        total_received += 1;
    }

    // 修复5：主线程等待所有子线程完全结束
    for handle in handles {
        handle.join().unwrap();
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}