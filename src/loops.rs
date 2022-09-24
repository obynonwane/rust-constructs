pub fn run(){
    let mut count = 0;

    //infinite loop
    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 20 {
            break;
        }
    }


    //while loop 
    while count <= 100 {
        if count % 15 == 0{
            println!("Fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else  {
            println!("{}", count);
        }

        println!("Number: {}", count);
        count += 1;
    }

    for x in 0..10  {
        if x % 15 == 0{
            println!("Fizzbuzz");
        } else if x % 3 == 0 {
            println!("fizz");
        } else if x % 5 == 0 {
            println!("buzz");
        } else  {
            println!("{}", x);
        }
    }
}