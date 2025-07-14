fn main() {
    let x = 48;
    let y = 18;
    let result = gcd(x, y);
    println!("{}", result);
  }
  
  pub fn gcd(x: u32, y: u32) -> u32 {
      let mut xa = 0;
      let mut ya = 0;
      for n in 1..(x+y) {
          let mut back:u32 = 0;
          back = (x+y) - n;
          if x % back == 0 && y % back == 0 {
              return back;
          }
      }
      return 500;
  } 
  