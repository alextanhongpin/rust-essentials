struct Score(i32, u8);

struct Kilogram(u32);

struct Player {
  name: &'static str,
  health: i32,
  level: u8
}

fn main() {

  let score = Score(32, 2);
  let Score(a, b) = score;
  println!("a: {}, b: {}", a, b);

  let kg = Kilogram(32);
  let Kilogram(weight) = kg;
  println!("weight is {}", weight);

  let mut player = Player{ name: "john", health: 100, level: 1 };
  player.name = "hoho";
  println!("player level {}", player.level);

  let Player{name: n, health:h, ..} = player;
  println!("player name {}", n);
  println!("player health {}", h);

  let ps = &Player{name: "jane", health: 97, level: 2};
  println!("{} == {}", ps.name, (*ps).name);
  println!("health and level {} {}", ps.health, ps.level);
}