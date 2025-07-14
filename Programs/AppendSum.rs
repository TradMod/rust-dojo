fn main() {
	let v = vec![];
	let result = append_sum(v);
	println!("{:?}", result);
}

pub fn append_sum(v: Vec<i32>) -> Vec<i32> {
    let mut vec: Vec<i32> = v.clone();
    if v.len() == 0 {
        vec.push(0);
        return vec;
    }
    let mut sum = 0;
    for i in 0..v.len() {
        sum += v[i];
    }
    vec.push(sum);
    return vec;
} 