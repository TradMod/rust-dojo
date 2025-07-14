fn main() {
	let v = vec![1, -2, 3, -4];
	
	let result = absolute_value(&v);
	println!("{:?}", result);
}

pub fn absolute_value(v: &Vec<i32>) -> Vec<i32> {
	let mut my_v = vec![];
	
	for i in 0..v.len(){
        if v[i] < 0 {
            my_v.push(v[i]*-1);
        } else {
            my_v.push(v[i]);
        }
    }
	my_v
} 