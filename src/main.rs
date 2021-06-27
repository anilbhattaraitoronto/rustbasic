

fn main() {
    let s1 = String::from("Hello world");
    let len = calculate_length(&s1); //reference
    println!("The length of s1 is {}", len);
    println!("The s1 says: {}", s1); 
    //dereferencing is used with *
    /* References are immutable
        therefore, cannot be modified
    */
}

fn calculate_length(item: &String) -> usize {
    item.len()
    //& is reference
}
