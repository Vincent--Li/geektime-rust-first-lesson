use std::fs::File;
use std::io::{BufWriter, Write};
use std::net::TcpStream;

#[derive(Debug)]
struct MyWriter<W> {
    writer: W,
}

impl<W: Write> MyWriter<W> {
    pub fn write(&mut self, buf: &str) -> std::io::Result<()> {
        self.writer.write_all(buf.as_bytes())
    }
}

impl MyWriter<BufWriter<TcpStream>> {
    pub fn new(addr: &str) -> Self {
        let stream = TcpStream::connect(addr).unwrap();
        Self {
            writer: BufWriter::new(stream),
        }
    }
}

impl MyWriter<File> {
    pub fn new(addr: &str) -> Self {
        let file = File::open(addr).unwrap();
        Self { writer: file }
    }
}

fn main() {
    let mut writer = MyWriter::<BufWriter<TcpStream>>::new("127.0.0.1:8080");
    let _ = writer.write("hello world!");

    let mut writer = MyWriter::<File>::new("test.txt");
    let _ = writer.write("hello world!");
}