fn main() {
	let v1 = vec![1,2,3,4];
	let v2 = vec![0,1,0,4];
	
	let result = elementwise_add(v1, v2);
	println!("{:?}", result);
}

pub fn elementwise_add(v1: Vec<i32>, v2: Vec<i32>) -> Vec<i32> {
    let mut vecnew:Vec<i32>=vec![];
    for i in 0..v1.len() {
        vecnew.push(v1[i] + v2[i]);
    }
    return vecnew;
} 