fn main() {
  let max_power = 10;
  let mut power = 1;
  while power < max_power {
    println!("{}", power);
    // Rust does not have ++
    power += 1;
  }

  // loop starts an infinite loop
  loop {
    power += 1;
    if power == 42 {
      // skips the rest of this iteration
      continue;
    }
    println!("power {}", power);

    if power == 50 {
      println!("ok, that's enough!");
      break; // exit the loop
    }
  }

  'outer: loop {
    println!("Entered the outer dungeon");
    'inner: loop {
      println!("Entered the inner dungeon");
      // break; // this will break out of the inner loop 
      break 'outer; // breaks to the outer loop
    }
    // println!("This treasure can sadly never be reached")
  }
  println!("Exited the outer dungeon");

  for n in 1..11 {
    println!("the square of {} is {}", n, n * n);
  }

  let mut x = 10;
  for _ in 1..x { x -= 1; println!("{}", x);}
}