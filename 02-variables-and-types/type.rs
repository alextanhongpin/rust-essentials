fn main() {
  let player1 = "rob";
  let player2 = "john";

  let players = player1.to_owned() + player2;
  println!("All players: {}", players);

  let players2 = player1.to_string() + player2;
  println!("All players2: {}", players2);

  let players3 = format!("{} {}", player1, player2);
  println!("All players3: {}", players3);

  let points = 10i32;
  let mut saved_points: u32 = 0;
  println!("You got {} points", saved_points);
  saved_points = points as u32;
  println!("You got {} points", saved_points);
}