
#[derive(Debug)]
struct BufBuilder {
    buf: Vec<u8>,
}


impl BufBuilder {
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: Vec::with_capacity(capacity)
        }
    }
}





fn main() {
    println!("Hello, world!");
}
