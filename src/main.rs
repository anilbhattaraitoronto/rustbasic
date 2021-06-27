

fn main() {
    let text = "hello";
    let num: u8 = 8;
    let mut user_text = String::from(text);
    user_text.push_str(", world!");
    println!("{}",user_text);
    takes_ownership(user_text);
    // println!("{}", user_text);
    //this will throw compile-time error coz the value has moved
    makes_copy(num);
    println!("{}", num);
    //This is fine because makes_copy copies

    let hello_text = gives_ownership();

    let some_text = String::from("Hello");
    let another_text= takes_and_gives_back(some_text);
    println!("The hello is {} and another is {}", hello_text, another_text);
    let length  = calculate_length(another_text).1;
    println!("The length is {}", length);
    
    
}

fn takes_ownership(some_string:String){
    println!("{}", some_string);
}

fn makes_copy(some_integer: u8){
    println!("{}", some_integer);
}

fn gives_ownership() -> String{
    let some_string = String::from("My world");
    some_string
}

fn takes_and_gives_back(a_string:String) -> String{
    a_string
}

fn calculate_length(s: String) -> (String, usize){
    let length = s.len();
    (s, length)
}