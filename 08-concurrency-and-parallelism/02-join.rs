use std::thread;
fn main() {
    let handle = thread::spawn(move || {
        println!("Hello, I'm called from the thread");
    });
    // do other work in the meantime
    let output = handle.join().unwrap();
    println!("{:?}", output);
}