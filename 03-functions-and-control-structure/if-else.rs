fn main() {
  let dead = false;
  let health = 14;
  if dead {
    println!("game over!");
  }
  if dead {
    println!("game over 2!");
    return
  }

  if health >= 50 {
    println!("continue to fight!");
  } else if health >= 20 {
    println!("stop the battle!");
  } else {
    println!("hide to recover");
  }

  let active = if health > 10 {true} else {false};
  println!("is active: {}", active);
}