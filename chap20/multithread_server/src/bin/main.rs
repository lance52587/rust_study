use std::thread;
use std::time::Duration;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;// prelude表示预导入，这里导入了io模块的所有内容
use std::fs;// 用于读取文件
use multithread_server::ThreadPool;

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 512]; // ;表示数组中所有元素都是0，512表示数组长度。这里定义了一个512字节的缓冲区

    stream.read(&mut buffer).unwrap();// read方法读取流中的数据，并将其存储到buffer中，返回值是Result类型，所以用unwrap方法处理错误

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else{
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let content = std::fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, content);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


fn main() {
    let listener = TcpListener::bind("localhost:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // println!("Connection established!");
        // handle_connection(stream);
        // thread::spawn(|| {// spawn方法创建一个新线程，并在新线程中运行闭包中的代码
        //     handle_connection(stream);
        // });
        pool.execute(|| {// pool.execute的接口与thread::spawn的完全一致，它会接收一个处理所有流的闭包
            handle_connection(stream);
        });
    }
}