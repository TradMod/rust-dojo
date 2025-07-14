fn main() {
    let n = 10;
    let result = fibonacci(n);
    println!("{}", result);
}

pub fn fibonacci(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return 1;
    }
        let mut first = 0;
        let mut second = 1;
    for i in 1..n+1 {
        let fib = first + second;
        first = second;
        second = fib;
        if i == n {
            return fib;
        }
    }
    return 500;
}