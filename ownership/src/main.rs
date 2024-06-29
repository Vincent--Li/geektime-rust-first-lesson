fn main() {
    let mut data = 1;
    let v = &mut data;
    *v += 1;
    println!("data: {:?}", &data);
    println!("data: {:?}", v1);
}