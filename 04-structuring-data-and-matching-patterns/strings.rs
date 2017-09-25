fn main() {
  let mut str1 = String::new();

  let c1 = '0';
  str1.push(c1);
  println!("str1 is {}", str1);
  str1.push_str("level 1 is finished - ");
  println!("str1 is {}", str1);
  str1.push_str("level 2 is finished");
  println!("str2 is {}", str1);

  let magician = "merlin";

  for c in magician.chars() {
    println!("{}", c);
  }

  for word in str1.split(" ") {
    println!("{}", word);
  }

  let str5 = str1.replace("level", "floor");
  println!("{}", str5);

  // Pass the string as slice as string allocates memory
  println!("the length is {}", how_long(&str5));
  println!("the length is {}", how_long(&str5[..]));
}

fn how_long(s: &str) -> usize { s.len() }
