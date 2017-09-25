fn main() {
  let location = "middle earth";
  let part = &location[7..12];
  println!("part is {}", part);

  let magician = "Merlin";
  let mut chars: Vec<char> = magician.chars().collect();
  chars.sort();
  for c in chars.iter() {
    println!("{}", c);
  }

  let v: Vec<&str> = "the wizard of oz".split(" ").collect();
  println!("v is {:?}", v);

  let u: Vec<&str> = "abc12de21d3r45f".split(|c: char| c.is_numeric()).collect();
  println!("u is {:?}", u);
}