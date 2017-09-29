#[derive(Debug)]
struct Pair<T> {
  first: T,
  second: T
}

#[derive(Debug)]
struct Person {
  name: &'static str,
  id: i32
}

fn main() {

  let magic_pair: Pair<u32> = Pair {first: 7, second: 42};
  println!("magic_pair is {:?}", second(magic_pair));

  let pair_of_magicians: Pair<&str> = Pair {first: "hello", second: "world"};
  println!("pair_of_magicians is {:?}", second(pair_of_magicians));


  let x: Option<i8> = Some(5);
  let pi: Option<f64> = Some(3.1415926);
  let none: Option<f64> = None;
  let none2 = None::<f64>;
  let name: Option<&str> = Some("joyce");
  println!("{:?} {:?} {:?} {:?} {:?}", x, pi, none, none2, name);

  let p1 = Person{name: "john doe", id: 7};
  let op1: Option<Person> = Some(p1);
  println!("person is {:?}", op1);
  match op1 {
    Some(Person{name: n, id: _}) => println!("match person is {:?}", n),
    None => println!("no person found")
  }
}

fn second<T>(pair: Pair<T>) -> T {
  pair.second
}