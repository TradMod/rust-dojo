fn main() {
	let v = vec![1,2,3];
	
	let result = remove_max(&v);
	
	println!("{:?}", &v);
	println!("{:?}", &result);
}

pub fn remove_max(v: &Vec<u32>) -> Vec<u32> {
    if v.len() <= 1 {
        return Vec::new();
    }
    
    let mut result = v.clone();
    
    let mut max = v[0];
    let mut max_idx = 0;
    
    for i in 0..v.len() {
        if v[i] > max {
            max_idx = i;
            max = v[i];
        }
    }
    
    result.remove(max_idx);
    
    result
}