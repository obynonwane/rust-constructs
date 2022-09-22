pub fn run(){
    //print to console
    println!("Hello from the print.rs file");

    //basic formatting
    println!("{} is from {}", "Obinna", "Mass");

    //positional formatting - Argument
    println!("{0} is from {1} and {0} like to {2}", "Obinna", "mass", "code");

    //named argument
    println!("{name} like to play {activity}", name="Obinna", activity="baseball");

    //placeholder traits
    println!("Binary: {:b} Hex: {:x} Octo: {:o}", 10, 10, 10);

}