use std::{io::Write, str::FromStr};

use regex::Regex;

#[derive(Debug)]
struct BufBuilder {
    buf: Vec<u8>,
}

impl BufBuilder {
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: Vec::with_capacity(capacity),
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
    type Error;
    fn parse(str: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

impl<T> Parse for T
where
    T: Default + FromStr,
{
    type Error = String;
    fn parse(str: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        if let Some(captures) = re.captures(str) {
            captures
                .get(0)
                .map_or(Err("failed to capture".to_string()), |a| {
                    a.as_str()
                        .parse()
                        .map_err(|_err| "failed to parse captured string".to_string())
                })
        } else {
            Err("failed to parse string".to_string())
        }
    }
}

#[test]
fn parse_should_work() {
    assert_eq!(u32::parse("123abcd"), Ok(123));
    assert_eq!(
        u32::parse("123.45abcd"),
        Err("failed to parse captured string".into())
    );
    assert_eq!(f64::parse("123.45abcd"), Ok(123.45));
    assert!(f64::parse("abcd").is_err());
}

fn main() {
    let mut buf = BufBuilder::new(1024);
    buf.write_cust();
    println!("result: {:?}", u8::parse("255 hello world"));
}
