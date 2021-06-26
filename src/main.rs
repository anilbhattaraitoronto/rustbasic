
fn main() {
    // let mut counter =0;
    // let result = loop{
    //     counter +=1;
    //     if counter == 10 {
    //         break counter *2;
    //     }
       
    // };
    // println!("The result is {}", result);
    // let mut number = 20;
    // while number !=0{
    //     println!("Hello number {}", number);
    //     number -=1;
    // }
    // println!("While it is crazy!")

    let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Other day"];
    let len_of_days = days.len();
    let mut index =0;

    while index < len_of_days {
        println!("They day is: {}", days[index]);
        index += 1;
    };

    for day in days.iter(){
        println!("Today is: {}", day);
    };
    for num in 1..15{
        println!("The number is: {}", num);
    }
    
}
