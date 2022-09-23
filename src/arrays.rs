/**
 * Arrays are fixed length where element are same data types
 */
use std::mem;
pub fn run(){
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    //reassign a value
    numbers[2] = 20;

    //printing using debug trait - to print arrau
    println!("{:?}", numbers);

    //get single value`
    println!("{:?}", numbers[0]);

    //Get array length
    println!("Array Length: {}", numbers.len());

    //Arays are stack alllocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    //get slice 
    let slice: &[i32] = &numbers[1..3];

    println!("slice {:?}", slice);
}