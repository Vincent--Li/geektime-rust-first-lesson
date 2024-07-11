use std::thread;

fn main() {
    let a = "Welcome to rust";

    let handle = thread::spawn(move || {
        println!("moved: {:?}", a);
    });

    handle.join().unwrap();
}
