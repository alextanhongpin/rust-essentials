#[derive(Debug)]
struct Alien {
  name: &'static str,
  damage: u32
}

trait Monster {
  fn new(&'static str, u32) -> Self;
  fn attack(&self);
}

impl Monster for Alien {
  fn new (n: &'static str, d: u32) -> Alien {
    return Alien {name: n, damage: d }
  } 
  fn attack(&self) {
    println!("I attack, I lower your health points by {}", self.damage);
  }
}

fn main() {
  let alien: Alien = Alien::new("x", 12);
  // let alien: Alien = Alien { name: "x", damage: 32 };
  alien.attack();
  println!("{:?}", alien);
}