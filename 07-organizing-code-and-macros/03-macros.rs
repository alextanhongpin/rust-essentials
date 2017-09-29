macro_rules! welcome {
  () => {
    println!("welcome!");
  }
}
macro_rules! greet {
  ($arg:expr) => {
    println!("greet {}", $arg);
  }
}

macro_rules! scream {
  ($($arg:expr), *) => ({
    $ (print!("scream {} /", $arg)); *
  })
}

fn main() {
  welcome!();
  greet!(32);
  greet!("hello");
  scream!(2, "asd");
}