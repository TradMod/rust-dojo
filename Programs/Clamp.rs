fn main() {
	println!("100 clamped: {}", clamp(100));
	println!("150 clamped: {}", clamp(150));
	println!("-100 clamped: {}", clamp(-100));
	println!("-200 clamped: {}", clamp(-200));
	println!("127 clamped: {}", clamp(127));
	println!("-128 clamped: {}", clamp(-128));
}

pub fn clamp(x: i32) -> i8 {
	if x > i8::MAX as i32 {
        return i8::MAX;
    } else if x < i8::MIN as i32 {
        return i8::MIN;
    } else {
        x as i8
    }
} 