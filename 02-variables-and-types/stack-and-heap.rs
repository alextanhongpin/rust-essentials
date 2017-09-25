fn main() {
  let health = 32;
  let game = "Space Invaders";
  println!("health pointer: {:p}", &health);
  println!("health value: {}", health);
  println!("game pointer: {:p}", &game);
  println!("game value: {}", game);

  // Error: cannot borrow mutably
  // let game2 = &mut game; 
  // println!("game2 pointer is {:p}", game2);
  let mut mut_game = "mutable Space Invaders";
  println!("original pointer is {:p}", &mut_game);
  let game2 = &mut mut_game;
  println!("game2 pointer is {:p}", game2);
  *game2 = "I modify you!";
  println!("game2 value is {}", game2);
}