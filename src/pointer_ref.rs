pub fn run(){
    //primitive array
    let arr1 = [1,2,3,4];
    let arr2 = arr1;

    //refernce types - vectors 
    let vec1 = vec![1,2,3,4];
    let vec2 = arr1;
    
    println!("primitive array {:?}", (vec1 , vec2));
}