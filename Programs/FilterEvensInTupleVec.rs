pub fn clone_filter_update(input: &Vec<(i32, bool)>) -> Vec<(i32, bool)> {
    let cloned = input.clone();
    let mut result = Vec::new();

	if cloned.len() == 0 {
        return cloned;
    } 
    for i in 0..cloned.len() {
        if cloned[i].0 % 2 == 0 {
            result.push((cloned[i].0, true));
        }
    }

    result
}

fn main() {
    let input = vec![(3, false), (3, false), (3, false)];
    println!("Original: {:?}, Filtered: {:?}", &input, clone_filter_update(&input) );
}