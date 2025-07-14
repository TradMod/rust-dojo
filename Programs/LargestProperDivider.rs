fn main() {
    let x = 12;
    let result = largest_proper_divisor(x);
    println!("{}", result);
  }
  
  pub fn largest_proper_divisor(x: u32) -> u32 {
    for n in 1..x {
        let backward_n = x - n;
        if x % backward_n == 0 {
          return backward_n;
        }
    }
    return 1;
  } 
  