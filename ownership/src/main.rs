use std::cell::RefCell;

fn main() {
    let data = RefCell::new(1);

    let mut v = data.borrow_mut();

    *v += 1;

    println!("data:{:?}", data.borrow()); // RefCell 静态编译能通过. 动态运行的时候, 会报错 违反了借规则"

}
