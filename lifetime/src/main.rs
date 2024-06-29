pub fn strtok<'a>(s: &'a mut & str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[i + delimiter.len_utf8()..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

fn main() {
    let s = "hello world".to_owned();
    let mut s1 = s.as_str(); // 所有者
    let hello = strtok(&mut s1, ' ');
    // println!("hello is: {}, s1: {}, s: {}", hello, s1, s); // hello 和 s1 生命周期绑定, hello使用结束前, s1 和 hello 一个&mut vs 一个 &. 同时存在
    println!("hello is: {}", hello);
    println!("s1 is: {}", s1);
}
