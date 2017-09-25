fn main() {
  let rng = 0..1000;
  // let rngvec = rng.collect::<Vec<i32>>();
  let rngvec: Vec<i32> = rng.collect();
  println!("{:?}", rngvec);

  let mut rng2 = 0..1000;
  let forty_two = rng2.find(|n| *n >= 42);
  println!("forty two and above {:?}", forty_two);

  let even: Vec<i32> = rng2.filter(|n| n % 2 == 0)
  .map(|n| n * n * n)
  .take(5)
  .collect();
  println!("even {:?}", even);

  let sum = (0..10).fold(0, |sum, n| sum + n);
  println!("sum is {}", sum);
}