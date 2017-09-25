fn main() {
  let a = 2;
  let b = 5;
  let c = a + b;
  println!("n is {}", c);

  let mut x = 0;
  let mut y = 1;
  let z = y; y = x; x = z;
  println!("x: {}, y: {}, z: {}", x, y, z); 

  let n1 = {
    let a = 2;
    let b = 3;
    a + b // <- no semicolon
  };
  println!("n1 is {}", n1);

  let n2 = {
    let a = 2;
    let b = 3;
    a + b; // surpress return value with semicolon
  };
  println!("n2 is {:?}", n2);
}