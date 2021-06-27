

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

    let third_var = String::from("This is another variable");
    let fourth_var = third_var.clone();

    println!("The third variable is: {}", third_var);
    println!("The fourth variable is: {}", fourth_var);

    //copying numerical variables

    let x = 5;
    let y = x;

    println!("The value of x is {}, and that of y is {}", x, y);
    
}
