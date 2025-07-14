fn main() {
    let x = 42;
    let y = 2;
    let op = 0;
    let result = calculator(x, y, op);
    println!("{}", result);
}

pub fn calculator(x: i32, y: i32, op: i32) -> i32 {
    if op == 0 {
        return x+y;
    } else if op == 1 {
        return x-y;
    } else if op == 2 {
        return x*y;
    } else if op == 3 {
        return x/y
    } else {
        return x%y;
    }
} 