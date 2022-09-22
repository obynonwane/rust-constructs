
/**
 * Primitive Str: Fixed length, Immutable string somewhere in memory
 * String = Growable, aheap allocated data-structure, used when you need to own or modify string data
 */
pub fn run(){

    let mut hello = String::from("Hello ");

    //get length of string
    println!("Length: {}", hello.len());

    //push character
    hello.push('W');

    //push string literal
    hello.push_str("orld");

    //capacity in bytes
    println!("Capacity: {}", hello.capacity());

    //is empty check
    println!("Is empty: {}", hello.is_empty());

    //constains substring
    println!("Constains 'World' {}", hello.contains("World"));

    //replace 
    println!("Replace 'World' {}", hello.replace("World", "there"));

    //loop through strings by white space
    for word in hello.split_whitespace(){
        println!("{}", word);
    }

    //create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);


    //Assertion Testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());



    println!("{}", hello);

}