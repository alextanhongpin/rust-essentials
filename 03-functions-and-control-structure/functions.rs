fn main() {
  let hero1 = "pac man";
  let hero2 = "riddick";

  greet(hero1);
  greet_both(hero1, hero2);

  let power = increment_power(1000);
  println!("my power is now {}", power);

  println!("the absolute of 0 is {}", absolute(-1));
}

fn greet(name: &str) {
  println!("hero name is {}", name);
}

fn greet_both(name1: &str, name2: &str) {
  println!("{} and {}", name1, name2);
  greet(name1);
  greet(name2);
}

fn increment_power(power: i32) -> i32 {
  println!("my power is going to increase");
  if power < 100 { return 999; }
  power + 1
}

fn absolute(x: i32) -> i32 {
  if x > 0 {
    x
  } else {
    -x
  }
}