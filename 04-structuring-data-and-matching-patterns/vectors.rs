fn main() {
  let mut numbers: Vec<i32> = Vec::new();
  let magic_numbers = vec![7i32, 42, 37, 100];
  println!("numbers {:?}", numbers);
  println!("magic_numbers: {:?}", magic_numbers);

  // Make a new vector and allocate an initial memory size to it
  let ids: Vec<i32> = Vec::with_capacity(25);
  println!("ids {:?}", ids);

  let rgvec: Vec<u32> = (0..7).collect();
  println!("collected {:?}", rgvec);

  let values = vec![1,2,3];
  for v in values {
    println!("v is {}", v);
  }

  numbers.push(magic_numbers[0]);
  numbers.push(magic_numbers[1]);
  println!("numbers: {:?}", numbers);
  let last = numbers.pop();
  println!("numbers: {:?}", numbers);
  println!("last {:?}", last);

  let slc = &magic_numbers[1..3];
  println!("slice is {:?}", slc);
}