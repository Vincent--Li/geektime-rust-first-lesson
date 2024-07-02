use std::fs::File;
use std::io::{BufWriter, Write};
use std::net::TcpStream;

#[derive(Debug)]
struct MyWriter<W> {
    writer: W,
}

impl<W: Write> MyWriter<W> {

    pub fn new(writer: W) -> Self {
        MyWriter { writer }
    }

    pub fn write(&mut self, buf: &str) -> std::io::Result<()> {
        self.writer.write_all(buf.as_bytes())
    }
}



fn main() {
    let mut writer = MyWriter::new(BufWriter::new(TcpStream::connect("127.0.0.1:8080").unwrap()));
    let _ = writer.write("hello world!");
}