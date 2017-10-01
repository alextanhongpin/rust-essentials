use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender,Receiver};
fn main() {
  let (tx, rx): (Sender<i32>, Receiver<i32>) = channel();
}