use std::mem::{align_of, size_of};

struct S1 {
    a: u8,
    b: u16,
    c: u8,
}

struct S2 {
    a: u8,
    c: u8,
    b: u16,
}

fn main() {
    println!("size of S1: {}, S2 {}", size_of::<S1>(), size_of::<S2>());
    println!("align of S1: {}, S2 {}", align_of::<S1>(), align_of::<S2>());
}
