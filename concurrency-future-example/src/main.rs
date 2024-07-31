use std::thread;

use tokio::{ 
    net::TcpListener, 
    sync::{
        mpsc, 
        oneshot
    },
};
use tokio_utils::codec::{Framed, LinesCodec};

pub const PREFIX_ZERO: &[u8] = &[0, 0, 0];

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:8080"; 
    let listener = TcpListener::bind(addr).await?; 
    println!("listen to: {}", addr);

    // 创建tokio task和 thread之间的channel
    let (sender, mut receiver) = mpsc::unbounded_channel::<(String, oneshot::Sender)>();

    // 使用thread处理计算密集型任务
    thread::spawn(move || {
        while let Some((line, reply)) = receiver

    });

    // 使用tokio task 处理IO密集型任务


}
