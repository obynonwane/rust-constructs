enum Movement {
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement) {
    //perform action depending on info
    //match functions just like switch
    match m {
        Movement::Down => println!("Avater moving Down"),
        Movement::Up => println!("Avater moving Up"),
        Movement::Left => println!("Avater moving Left"),
        Movement::Right => println!("Avater moving Right"),
    }
}
pub fn run(){
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Right;
    let avatar3 = Movement::Up;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);

}