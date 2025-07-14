fn main() {
    let n = 5;
    let result = factorial(n);
    println!("{}", result);
  }
  
  pub fn factorial(n: u32) -> u32 {
    if n == 0 || n == 1 {
      return 1;
    }
    let last_value = n;
    let mut sec_last_value = n-1;
    let mut facto = last_value * sec_last_value;
    for _i in 0..n {
        if sec_last_value == 1 {
          return facto;
        }
        sec_last_value = sec_last_value - 1;
        facto = facto * sec_last_value;
        println!("facto {}", facto);
    }
    return 500;
  } 