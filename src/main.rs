
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

    println!("Total is {}", total);
    println!("Difference is {} ", difference);
    println!("Multiples are {}", multiples);
    println!("Fractions are {}", fractions);

}
