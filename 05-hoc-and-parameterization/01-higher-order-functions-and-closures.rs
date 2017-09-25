fn main() {
  let mut strength = 27;
  println!("My tripled strength equals {}", triples(strength));
  strength = triples(strength);
  println!("My tripled strength equals {}", strength);
  let strength2 = again(triples, 27);
  println!("my strength is now {}", strength2);

  let strength3 = again(|n| n * 3, 27);
  println!("my strength is now {}", strength3);

  // closures
  let x: i32 = 42;
  let print_add = move |s| {
    println!("x is {}", x);
    x + s
  };
  let res = print_add(100);
  println!("res is {}", res);
}

fn triples(s: i32) -> i32 { s * 3 }

fn again <F: Fn(i32) -> i32>(f: F, s: i32) -> i32 { f(f(s)) }