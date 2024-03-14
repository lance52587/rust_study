use std::thread;
use std::time::Duration;

fn main() {
    // thread::spawn(|| {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));// sleep for 1 millisecond
        }
    });
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    };

    let v = vec![1, 2, 3];
    // let handle = thread::spawn(|| {
        let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // drop(v); // 情况不妙 // 由于move将v移动到了闭包的环境中，所以我们无法在主线程中继续使用它来调用drop函数了

    handle.join().unwrap();

}
