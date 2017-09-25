use std::io;

fn main() {
  println!("What's your name?");
  let mut buf = String::new();

  io::stdin().read_line(&mut buf)
  .ok()
  .expect("failed to read line");
  println!("That's a mighty name indeed, {}", buf.trim());

  println!("What's your age?");
  let mut buf2 = String::new();
  io::stdin().read_line(&mut buf2)
  .ok()
  .expect("failed to read number");

  let input_age: Result<u32,_> = buf2.trim().parse();

  match input_age {
    Ok(age) => println!("The age is good {}", age),
    Err(ex) => println!("Please input an integer number: {}", ex)
  }

  println!("What's your age again?");
  let mut buf3 = String::new();
  io::stdin().read_line(&mut buf3)
  .ok()
  .expect("failed to read number");

  let input_age2: Result<u32,_> = buf3.trim().parse();

  // let age = match input_age2 {
  //   Ok(age) => age,
  //   Err(_) => 0
  // };
  // println!("age is {}", age);

  if let Ok(val) = input_age2 {
    println!("age is {}", val);
  } else {
    println!("error getting age");
  }
}