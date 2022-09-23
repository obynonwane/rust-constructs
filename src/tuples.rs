/**
 * Tuples group together values, they can be  of same or different types
 * Max of 12 elements
 */
pub fn run(){
    let person:(&str, &str, i8) = ("Obinna", "Mass", 23);
    println!("{} is from {} and he is {} years old", person.0, person.1, person.2);
}