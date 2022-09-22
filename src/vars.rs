pub fn run(){
    //variable are immutable by default in rust
    //variable hold primitive data or reference data 
    //rust is a blocked scoped language

    let name = "Obinna";
    let mut age = 28;
    println!("my name is {} and I am  {}", name, age);
    age = 32;
    println!("my name is {} and I am  {}", name, age);

    const ID: i32 = 001;

    println!("ID is: {}", ID);

    //Assign multiple variable 
    let (my_name, my_age) = ("Obinna", 32);

    println!("my name is {} and I am {} old", my_name, my_age);
}