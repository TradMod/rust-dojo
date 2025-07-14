fn main() {
	let v = vec![1,2,3,2,1];
	
	let result = is_palindrome(v);
	println!("{}", result);
}

pub fn is_palindrome(v: Vec<i32>) -> bool {
    if v.len() == 0 || v.len() == 1 {return true;}
    let mut backwardloop = v.len()-1;
    for i in 0..v.len() {
        if v[i] == v[backwardloop] {
            if i == v.len()/2 {
                return true
            }
        } 
        if v[i] != v[backwardloop] { return false;}
        backwardloop = backwardloop-1;
    }
    false
} 