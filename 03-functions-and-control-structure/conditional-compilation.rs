fn main() {
  on_macos();
  // on_windows();
}

#[cfg(target_os = "macos")]
fn on_macos() {
  println!("you are running macos!")
}

// #[cfg(target_os = "windows")]
// fn on_windows() {
//   println!("you are running windows!")
// }