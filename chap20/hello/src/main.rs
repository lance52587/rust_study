use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;// prelude表示预导入，这里导入了io模块的所有内容
use std::fs;// 用于读取文件

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

    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));// from_utf8_lossy接收一个u8类型的slice，并返回一个String类型的值。这里的&buffer[..]是一个u8类型的slice
    // lossy表示如果slice中包含无效的UTF-8序列，将替换为U+FFFD REPLACEMENT CHARACTER

    let response = "HTTP/1.1 200 OK\r\n\r\n";// HTTP响应头
    // let contents = "Hello, world!";// HTTP响应体
    let contents = fs::read_to_string("hello.html").unwrap();// 读取文件内容

    let response = format!("{}{}", response, contents);// format!宏用于创建一个字符串，这里用于将响应头和响应体拼接成一个字符串

    stream.write(response.as_bytes()).unwrap();// stream的write方法只接收&[u8]类型值作为参数❸，所以我们需要调用response的as_bytes方法来将它的字符串转换为字节
    // 并将这些字节发送到连接中去
    stream.flush().unwrap();// flush调用会等待并阻止程序继续运行直到所有字节都被写入连接中❹；为了减少对底层操作系统的调用，TcpStream的实现中包含了一个内部缓冲区。
}