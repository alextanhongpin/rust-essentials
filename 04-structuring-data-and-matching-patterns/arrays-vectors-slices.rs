fn main() {
  // [type; number]
  let people: [&str; 4] = ["alex", "beta", "cat", "dot"];
  println!("{:?}", people);

  println!("the first person is {}", people[0]);
  println!("the second person is {}", people[1]);

  println!("the last person is {}", people[people.len() - 1]);
  println!("the last person is {}", people.iter().last().unwrap());

  let empty: [i32; 0] = [];
  println!("empty: {:?}", empty);

  let copy = &people;
  println!("the first copy is {}", copy[0]);
  // copy[0] = "halo"; can't do this, unless people is mutable

  let mut mut_people: [&str; 4] = ["alex", "beta", "cat", "dot"];
  mut_people[0] = "andrew";
  let copy2 = &mut_people;
  println!("mut_people is now {:?}", mut_people);
  println!("copy is now {:?}", copy2);

  // looping in a less performant way
  for ix in 0..people.len() {
    // because rust also needs to check whether we are still within the bounds of array in memory
    println!("found {}", people[ix]);
  }

  // better
  for p in people.iter() {
    println!("found {}", p);
  }

  // better and shorter
  for p in &people { println!("{}", p); }
}