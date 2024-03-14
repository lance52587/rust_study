// use std::sync::Mutex;
use std::sync::{Arc, Mutex};
// use std::rc::Rc;
// use std::thread;
use std::{thread, time::Duration};
use std::sync::mpsc;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap(); //lock方法来获取锁。这个调用会阻塞当前线程直到我们取得锁为止。
        *num = 6;
    }

    println!("m = {:?}", m);

    // let counter = Mutex::new(0);
    // let counter = Rc::new(Mutex::new(0));
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // let counter = Rc::clone(&counter);
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // 死锁案例
    let lock1 = Arc::new(Mutex::new(1));
    let lock2 = Arc::new(Mutex::new(2));

    let lock1a1 = Arc::clone(&lock1);
    let lock2a1 = Arc::clone(&lock2);
    let handel1 = thread::spawn(move || {
        println!("handel1 开始获取 lock1");
        let mut _a1 = lock1a1.lock().unwrap();
        println!("handel1 已获取 lock1 并使用");
        thread::sleep(Duration::from_secs(1));
        println!("handel1 开始获取 lock2");
        let _a2 = lock2a1.lock().unwrap();
        println!("handel1 已获取 lock1 和 lock2");
    });

    let lock1a2 = Arc::clone(&lock1);
    let lock2a2 = Arc::clone(&lock2);
    let handel2 = thread::spawn(move || {
        println!("handel2 开始获取 lock2");
        let mut _a2 = lock2a2.lock().unwrap();
        println!("handel2 已获取 lock2 并使用");
        thread::sleep(Duration::from_secs(1));
        println!("handel2 开始获取 lock1");
        let _a1 = lock1a2.lock().unwrap();
        println!("handel2 已获取 lock1 和 lock2");
    });

    handel1.join().unwrap();
    handel2.join().unwrap();
    println!("未发生死锁！");
}