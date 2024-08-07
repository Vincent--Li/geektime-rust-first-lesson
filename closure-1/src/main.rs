use std::{collections::HashMap, mem::size_of_val};

fn main() {
    // 长度为0
    let c1 = || println!("hello world!");
    // 和参数无关, 长度也为0
    let c2 = |i: i32| println!("hello: {}", i);
    let name = String::from("Vincent");
    let name1 = name.clone();
    let mut table = HashMap::new();
    table.insert("hello", "world");
    // 如果捕获一个引用, 长度为8
    let c3 = || println!("hello: {}", name);
    // 捕获移动的数据name1(长度24)  table(长度48), closure 长度72
    let c4 = move || println!("hello: {}, {:?}", name1, table);
    let name2 = name.clone();
    // 和局部变量无关, 捕获了一个String name2, closure 长度24
    let c5 = move || {
        let x = 1;
        let name3 = String::from("lindsey");
        println!("Hello: {}, {:?}, {:?}", x, name2, name3);
    };

    // 闭包的大小跟参数, 局部变量都无关, 只跟捕获的变量有关
    println!(
        "c1: {}, c2: {}, c3: {}, c4: {}, c5: {}, main: {}",
        size_of_val(&c1),
        size_of_val(&c2),
        size_of_val(&c3),
        size_of_val(&c4),
        size_of_val(&c5),
        size_of_val(&main),
    )

}
