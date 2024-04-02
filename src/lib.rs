 pub fn square(s: u32) -> Result<u64, String> {
  if s < 1 || s > 64 {
    Err(String::from("Square must be between 1 and 64"))
  } else {
    Ok(1 << (s - 1)) // Optimized calculation using bit shift
  }
}

pub fn total() -> u64 {
  // Optimized calculation to avoid overflow
  (1 << 64) - 1
}

fn main() {
  // Example usage
  match square(3) {
    Ok(grains) => println!("Square 3: {}", grains),
    Err(err_msg) => println!("{}", err_msg),
  }

  println!("Total grains: {}", total());
}
