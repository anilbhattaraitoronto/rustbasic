
fn main() {
    let first_num = 8;
    println!("The first number is {}", first_num);
    let mut second_num = 9;
    println!("The second number is {}", second_num);

    second_num = 12;
    println!("The reassigned second number is {}", second_num);
    const AGE: u32 = 51;
    println!("My age is {}", AGE);

    let spaces = "    ";
    let spaces = spaces.len(); //shadowing of variable

    println!("The number of spaces is {}", spaces);
    // Data Types

    let my_share: f32 = 3.4;
    let speed: f64 = 5.670986;
    let my_age: u8 = 51;
    let earth_age: u64 = 4_800_000_000;

    println!("My share is {}", my_share);
    println!("Speed limit is: {} ", speed);
    println!("My age is: {} ", my_age);
    println!("The earth's age is {}", earth_age);

    //number operations

    let total = 5 + 6;
    let difference = 20.56 - 19.08;
    let multiples = 5 * 6;
    let fractions: f32 =  25.00 / 100.00;
    let remainder = 67%8;

    println!("Total is {}", total);
    println!("Difference is {} ", difference);
    println!("Multiples are {}", multiples);
    println!("Fractions are {}", fractions);
    println!("Remainder is {}", remainder);

    //Boolean types

    let is_driver = true;
    let is_mechanic: bool = false;

    println!("He is a driver, right? {}", is_driver);
    println!("Is he a mechanic? {}", is_mechanic );

    //char types

    let first_char = 'a';
    let second_char = 'b';
    println!("First character is {} and second character is {}", first_char, second_char);

    //Compound types: tuple and array

    //tuple 
    // name: (type1, type2, type3) = (val1, val2, val3);

    let days = ("Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday");
    let (d1, d2, d3, d4, d5, d6, d7) = days; //destructuring

    println!("Days of the week are: {}, {}, {}, {}, {}, {}, {}", d1, d2, d3, d4, d5, d6, d7);
    println!("Firs day is: {}", days.0);

    //array
    // name: [type1, type2, type3] = [val1, val2, val2];
    //useful for stack and, like tuple, also is fixed;

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let one_to_five: [i8; 5] = [1, 2, 3, 4, 5];

    println!("The first month is : {}", months[0]);
    println!("The first number is: {}", one_to_five[0]);

    print_my_name();

}

fn print_my_name(){
    println!("My name is Anil Bhattarai")
}
