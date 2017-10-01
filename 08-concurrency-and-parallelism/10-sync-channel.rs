use std::sync::mpsc::sync_channel;
use std::{thread, time};

type TokenType = i32;
struct Msg {
  typ: TokenType,
  val: String
}

fn main() {
  let (tx, rx) = sync_channel(1); // Buffer size 1
  tx.send(Msg { typ: 42, val: "rust is cool".to_string()}).unwrap();
  println!("message 1 send");
  thread::spawn(move || {
    tx.send(Msg{ typ: 43, val: "rust is still cool!".to_string()}).unwrap();
    println!("message 2 send");
  });

  println!("waiting for 3 seconds");
  let three_seconds = time::Duration::new(3, 0);
  thread::sleep(three_seconds);
  if let Some(msg) = rx.recv().ok() {
    println!("received message of type {} and val {}", msg.typ, msg.val);
  };
  if let Some(msg) = rx.recv().ok() {
    println!("received second message of type {} and val {}", msg.typ, msg.val);
  };
}