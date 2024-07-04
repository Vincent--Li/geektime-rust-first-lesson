use std::io::Write;

use regex::Regex;



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

    pub fn write_cust(&mut self) {
        println!("test write cust");
    }
}

impl Write for BufBuilder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub trait Parse {
    fn parse(str: &str) -> Self;
}

impl Parse for u8 {
    fn parse(str: &str) -> Self {
        let re = Regex::new(r"^[0-9]+").unwrap();
        if let Some(captures) = re.captures(str) {
            captures.get(0).map_or(0,  |s| s.as_str().parse().unwrap_or(0))
        } else {
            0
        }
    }
}

#[test]
fn parse_should_work() {
    assert_eq!(u8::parse("123abcd"), 123);
    assert_eq!(u8::parse("1234abcd"), 0);
    assert_eq!(u8::parse("abc"), 0);
}

fn main() {
    let mut buf = BufBuilder::new(1024);
    buf.write_cust();
}
