use std::{thread, time};
fn main() {
  let mut health = 12;
  for i in 2..5 {
    thread::spawn(move || {
      health *= i;
    });
  }
  let two_seconds = time::Duration::new(2, 0);
  thread::sleep(two_seconds);
  println!("{}", health);
}