use std::{fmt, ops::Deref, str};

const MINI_STRING_MAX_LEN: usize = 30;

struct MiniString {
    len: u8,
    data: [u8; MINI_STRING_MAX_LEN],
}

impl MiniString {
    // new 接口不暴露出去, 保证传入的v的字节长度小于等于30
    fn new(v: impl AsRef<str>) -> Self {
        let bytes = v.as_ref().as_bytes();
        // 拷贝内容的时候, 一定要使用字符串的字节长度
        let len = bytes.len();
        let mut data = [0u8; MINI_STRING_MAX_LEN];
        data[..len].copy_from_slice(bytes);
        Self {
            len: len as u8,
            data,
        }
    }
}

impl Deref for MiniString {
    type Target  = str;

    fn deref(&self) -> &Self::Target {
        // 生成MiniString的接口是隐藏的, 只能来自字符串, 所以下面是安全的
        str::from_utf8(&self.data[..self.len as usize]).unwrap()
        // 也可以直接用unsafe版本
        // unsafe { str::from_utf8_unchecked(&self.data[..self.len as usize]) }
    }
}

impl fmt::Debug for MiniString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

#[derive(Debug)]
enum MyString {
    Inline(MiniString),
    Standard(String),
}

// 实现Deref 接口对两种不同的场景统一得到 &str
impl Deref for MyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            MyString::Inline(s) => s.deref(),
            MyString::Standard(s) => s.deref(),
        }
    }
}

impl From<&str> for MyString {
    fn from(s: &str) -> Self {
        match s.len() > MINI_STRING_MAX_LEN {
            true => MyString::Standard(s.to_owned()),
            _ => MyString::Inline(MiniString::new(s)),
        }
    }
}

impl fmt::Display for MyString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}



fn main() {
    let len1 = std::mem::size_of::<MyString>();
    let len2 = std::mem::size_of::<MiniString>();
    println!("Len: MyString: {}, MiniString: {}", len1, len2);

    let s1: MyString = "hello world".into();
    let s2: MyString = "这是一个超过了三十字节的很长很长的字符串".into();

    // debug 输出
    println!("s1: {:?}, s2: {:?}", s1, s2);
    // display 输出
    println!(
        "s1: {}({} bytes, {} chars), s2: {}({} bytes, {} chars)",
        s1,
        s1.len(),
        s1.chars().count(),
        s2,
        s2.len(),
        s2.chars().count(),
    );

    assert!(s1.ends_with("world"));
    assert!(s2.starts_with("这"));
}