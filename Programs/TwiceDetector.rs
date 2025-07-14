fn main() {
	let v = vec![1,2,1,4,5];
	let k = 1;
	
	let result = contains_k_twice(v, k);
	println!("{}", result);
}
pub fn contains_k_twice(v: Vec<i32>, k: i32) -> bool {
    let mut count = 0;
    for num in v {
        if num == k {
            count += 1;
            if count >= 2 {
                return true;
            }
        }
    }
    false
}