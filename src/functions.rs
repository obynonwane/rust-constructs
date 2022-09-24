pub fn run(){
    greetings("hello", "obinna");
    
    //Biond function values to variables;
    let get_sum = add(5, 7);
    println!("{}", get_sum);

    //closure 
    let n3: i32 = 4;
    let add_nums = |n1: i32, n2: i32| n1+ n2 + n3;
    println!("C Sum: {}", add_nums(3, 3));
}

fn greetings(greet: &str, name: &str){
    println!("{} {}, nice to meet you", greet, name );
}

fn add (n1: i32, n2: i32) -> i32 {
   return  n1 + n2;
}