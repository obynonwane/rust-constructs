/**
 * Primitive types----------------------------------------------------------------
 * integers u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 
 * Floats: f32, f64
 * Boolean (bool)
 * Characters (chars)
 * Tuples - basically list
 * Arrays - are fixed length - vectors
 */

 /**
  * Rust is statically typed language which means it must know the type of the
  variable at complile tume, the compiler can also infer on the type of a variable is not specified 
  based on the content or value
  */
pub fn run(){

    //default is i32
    let _x = 1;

    //add explicit type
    let _y: f64 = 2.5;

    //find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);


    //boolean
    let is_active = true;

    //get boolean from expression
    let is_greater = 10 > 5;

    //Character
    let a1 = 'a';

    let face = '\u{1F600}';

    println!("{:?}", (is_active, _y, _x, is_greater, a1, face));

}