fn main() {
  let thor =("thor", true, 3500u32);
  println!("{:?}", thor);

  println!("{} {} {}", thor.0, thor.1, thor.2);

  let (name, _, power) = thor;
  println!("name {} power {}", name, power);

  let (n, p) = increase_power(thor.0, thor.2);
  println!("name {} power {}", n, p);
}

fn increase_power(name: &str, power: u32) -> (&str, u32) {
  if power > 3000 {
    return (name, power * 2);
  } else {
    return (name, power * 3);
  }
}

