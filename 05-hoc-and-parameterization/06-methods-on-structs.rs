#[derive(Debug)]
struct Alien {
  health: u32,
  damage: u32
}

impl Alien {
  fn new(mut h: u32, d: u32) -> Alien {
    if h > 100 { h = 100; }
    Alien { health: h, damage: d }
  }
  fn cry() -> &'static str {
    "wohooo"
  }
  fn attack(&mut self) {
    self.health -= 15;
  }
}

fn main() {
  let mut berserk = Alien::new(150, 15);
  println!("{:?}", berserk);
  berserk.attack();
  println!("Got 15 damage: {:?}", berserk);

  println!("Alien cry: {}", Alien::cry());
}