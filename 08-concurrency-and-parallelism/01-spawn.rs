use std::{thread, time};
fn main() {
    // Closure without parameters ||
    thread::spawn(move || {
        println!("hello, I'm spawned from a thread")
    });
    let ten_millis = time::Duration::from_millis(50);
    thread::sleep(ten_millis);
}