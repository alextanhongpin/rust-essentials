// enum Compass {
//   North, South, West, East
// }
fn main() {
  // let direction = Compass::West;

  type Species = &'static str;

  let i: Species = "fooo";
  println!("{}", i);

  enum PlanetaryMonster {
    VenusMonster(Species, i32),
    MarsMonster(Species, i32)
  }

  let martian = PlanetaryMonster::MarsMonster("chela", 13);
}