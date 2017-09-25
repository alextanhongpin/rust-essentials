// Run the rustdoc doc.rs
fn main() {
  println!("the cube of 4 is {}", cube(4));
}

/// Calculates the cube `val * val * val`
///
/// # Example
/// ```
/// let cube = cube(val);
/// ```
pub fn cube(val: u32) -> u32 {
  val * val * val
}