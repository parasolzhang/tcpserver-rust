use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use std::thread;

fn main() {
    println!("Starting TcpServer ...");

    //绑定本地监听服务
    let listener = TcpListener::bind("127.0.0.1:7437").expect("Unable to bind to socket");
    //获取本地地址
    let addr = listener.local_addr().expect("Unable to get the local port");
    //控制台打印监听端口
    println!("listening the port: {}", addr.port());
    //循环获取连接客户端
    for connection in listener.incoming() {
        //使用match分支模式以及Err捕获模式
        match connection {
            Ok(stream) => {
                //开启线程并调用客户端消息处理逻辑
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(_) => {println!("Failed to process message.")},
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    //初始化链接时回传客户端，表示连接成功
    if let Err(_) = stream.write("connected success\n".as_bytes()){
        return;
    }
    println!("client connected");
    //读取客户端发送消息
    let mut buff = [0; 100];
    loop {
        //读取客户端消息至buff
        if let Ok(read) = stream.read(&mut buff) {
            if read == 0 {
                break;
            }
            //将buff消息打印到控制台
            if let Ok(msg) = std::str::from_utf8(&buff[0..read]) {
                println!("{}", msg);
            } else {
                break;
            }
        } else {
            break;
        }
    }
    //断开服务连接
    println!("client disconnected");
}