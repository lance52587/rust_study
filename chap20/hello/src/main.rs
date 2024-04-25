use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;// prelude表示预导入，这里导入了io模块的所有内容

fn main() {
    let listener = TcpListener::bind("localhost:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {// TcpStream的内部状态是可变的，所以这里要用mut
    let mut buffer = [0; 512]; // ;表示数组中所有元素都是0，512表示数组长度。这里定义了一个512字节的缓冲区

    stream.read(&mut buffer).unwrap();// read方法读取流中的数据，并将其存储到buffer中，返回值是Result类型，所以用unwrap方法处理错误

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));// from_utf8_lossy接收一个u8类型的slice，并返回一个String类型的值。这里的&buffer[..]是一个u8类型的slice
    // lossy表示如果slice中包含无效的UTF-8序列，将替换为U+FFFD REPLACEMENT CHARACTER
}