/**
 * Vectors are resizeable arrays are fixed length where element are same data types
 */
use std::mem;
pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    //reassign a value
    numbers[2] = 20;

    //add unto vector`
    numbers.push(6);
    numbers.push(7);

    //pop off the last value
    numbers.pop();

    //printing using debug trait - to print vector
    println!("{:?}", numbers);

    //get single value`
    println!("{:?}", numbers[0]);

    //Get vector length
    println!("Vector Length: {}", numbers.len());

    //Vector are stack alllocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    //get slice 
    let slice: &[i32] = &numbers[1..3];

    println!("slice {:?}", slice);

    //loop through vector values 
    for x in numbers.iter() {
        println!("Numbers {}", x);
    }

    //loop and mutate values
    for x in numbers.iter_mut(){
        *x *=2;
    }

    println!("Numbers Vec: {:?}", numbers);
}