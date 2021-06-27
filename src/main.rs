

fn main() {
    let mut var_one = String::from("Hello");
    // let my_text = String::from("World");
    var_one.push_str(", World");
    println!("The message is: {} ", var_one);
    let first_str = String::from("Ke Cha");
    println!("The length of first string is: {}", first_str.len());
    let second_str = first_str; //first_str is moved to second_str
    println!("The length of second string is: {} ", second_str.len());
    // println!("The first string is: {}", first_str);
    
}
