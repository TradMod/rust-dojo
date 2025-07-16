
pub fn update_tuple_elements(t: &(Vec<i32>, bool, i32)) -> (Vec<i32>, bool, i32) {
    let mut new_t = t.clone();
    new_t.0.push(1);
    if new_t.1 {
        new_t.1 = false;
    } else {
        new_t.1 = true;
    }
    new_t.2 = new_t.2 + 1;
    return new_t;
}

fn main() {
    let t = (vec![10, 20], true, 5);
    
    let updated_t = update_tuple_elements(&t);
    
    println!("Original: {:?}, Updated{:?}",t ,updated_t);
}
