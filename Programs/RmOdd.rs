fn main() {
	let v = vec![1,2,3,4];
	let result = remove_odd(v);
	println!("{:?}", result);
}

pub fn remove_odd(v:Vec<i32>) -> Vec<i32> {
    let mut vecnew :Vec<i32> = vec![];
    if v.len() == 0 {
        return vecnew;
    }
    for i in 0..v.len() {
        if v[i] % 2 == 0 {
            vecnew.push(v[i]);
        }
    }
    return vecnew;
}