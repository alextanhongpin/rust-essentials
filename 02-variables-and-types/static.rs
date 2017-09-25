static MAX_HEALTH: i32 = 100;
// Contains reference to string
static GAME_NAME: &'static str = "monster attack!";

fn main() {
  println!("hello, rust!");
  println!("The game you are playing is called {}.", GAME_NAME);
  println!("You start with {} health points.", MAX_HEALTH);

  println!("In this game {0} you start with {1} health points", GAME_NAME, MAX_HEALTH);
  println!("Oops, your health point drop to {point}", point=50);
}