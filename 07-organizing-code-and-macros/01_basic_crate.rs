mod game1 {
  // fn func1() {
  //   println!("Am I visible?");
  // }
  pub fn func2() {
    println!("You called func2 in game1");
  }
  pub mod subgame1 {
    pub fn subfunc1() {
      println!("This is another subfunc!");
    }
  }
  #[derive(Debug)]
  pub struct PrivateMagician {
    name: String, // Private fields
    age: i32,
    power: i32
  }
  impl PrivateMagician {
    pub fn new (name: String, age: i32, power: i32) -> Self {
      PrivateMagician { name: name.to_string(), age: age, power: power }
    }
  }
  #[derive(Debug)]
  pub struct PublicMagician {
    pub name: String, 
    pub age: i32,
    pub power: i32
  }
}

fn main() {
  // game1::func1();
  game1::func2();
  game1::subgame1::subfunc1();
  let mag1 = game1::PublicMagician { name: "Gandalf".to_string(), age: 25, power: 98 };
  println!("{:?}", mag1);

  let mag2 = game1::PrivateMagician::new("Gandalf".to_string(), 25, 89);
  println!("{:?}", mag2);
}