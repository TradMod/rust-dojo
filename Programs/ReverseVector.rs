fn main() {
	let v = vec![1,2,3,4];
	
	let result = reverse(v);
	println!("{:?}", result);
}

pub fn reverse(v: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    
    for i in 0..v.len() {
        result.push(v[v.len() - 1 - i]);
    }
    result
}