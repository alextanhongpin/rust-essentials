fn main() {
  let mut energy = 5;
  let copy_energy = energy;
  energy = 10;
  println!("You have {} energy, from {}", copy_energy, energy);

  let outer = 42;
  {
    let inner = 3.14;
    println!("Block variable {}", inner);
    let outer = 100;
    println!("Block variable outer {}", outer);
  }
  println!("Outer variable {}", outer);
}