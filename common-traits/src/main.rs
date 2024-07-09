#[derive(Clone, Debug)]
struct Developer {
    name: String, 
    age: u8,
    lang: Language
}

#[derive(Clone, Debug)]
enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell
}

fn main() {
    let dev = Developer {
        name: "John".to_string(),
        age: 30,
        lang: Language::Rust
    };
    // clone 是深拷贝, 栈内存和堆内存一起拷贝, 这也是为什么, 数据结构里的每个字段都需要实现过Clone
    let dev1 = dev.clone();
    println!("dev: {:?}, addr of dev name: {:p}", dev, dev.name.as_str());
    println!("dev1: {:?}, addr of dev1 name: {:p}", dev1, dev1.name.as_str());
}

