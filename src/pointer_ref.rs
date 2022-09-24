pub fn run(){
    //primitive array
    let arr1 = [1,2,3,4];
    let arr2 = arr1;

    //refernce types - vectors 
    let arr1 = vec![1,2,3,4];
    let arr2 = arr1;
    
    println!("primitive array {:?}", (arr1 , arr2));
}