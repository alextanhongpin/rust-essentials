fn main() {
  let mut rng = 0..7;
  println!("> {:?}", rng.next());
  println!("> {:?}", rng.next());

  for n in rng {
    println!("> {:?}", n);
  }
}