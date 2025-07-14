use std::collections::HashSet;

fn main() {
    let v1 = vec![1,2,3,1,4];
    let idx1 = return_index_of_first_duplicate(&v1);
    println!("Vector {:?} -> First dup index: {}", v1, idx1);

    let v2 = vec![1, 2, 5, 5, 3];
    let idx2 = return_index_of_first_duplicate(&v2);
    println!("Vector {:?} -> First dup index: {}", v2, idx2);

    let v3 = vec![1, 2, 3, 4, 5];
    let idx3 = return_index_of_first_duplicate(&v3);
    println!("Vector {:?} -> First dup index: {}", v3, idx3);

    let v4: Vec<i32> = vec![];
    let idx4 = return_index_of_first_duplicate(&v4);
    println!("Vector {:?} -> First dup index: {}", v4, idx4);
}

pub fn return_index_of_first_duplicate(v: &Vec<i32>) -> i32 {
    let mut set = HashSet::new();
    for i in 0..v.len(){
        let boo = set.insert(&v[i]);
        if !boo {
            return i as i32;
        }
    }
    -1
} 