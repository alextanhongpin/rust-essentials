fn main() {
  println!("no tests are compiled! compile with rustc --test")
}

#[test]
fn arimethic() {
  // This test will always pass
  // if 2 + 3 == 6 {
  //   println!("you can calculate")
  // }
  assert_eq!(5, 2 + 3);
}