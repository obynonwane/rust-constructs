pub fn run(){
    let age = 19;
    if age >= 21 {
        println!("What would you like to drink");
    } else {
        println!("Your age is below the required age")
    }

    //shorthand if
    let is_of_age = if age >= 21 {true} else {false};
    println!("Is of age {}", is_of_age);
}