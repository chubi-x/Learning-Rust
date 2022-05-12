use std::collections::HashMap;
fn main(){
    let mut map = HashMap::new();
    map.insert(0,"hello");
    map.insert(1,"world");
    // print the map
    println!("This is a hash map : {:?}", map);

    match map.get(&0){
        Some (str) => println!("{str}"),
        None =>  println!("This value doesn't exist in the map")

    }
    match map.get(&2){
        Some (str) => println!("{str}"),
        None => println!("This value doesn't exist in the map")
    }
    // map.remove(&0);
    // println!("This is the new value of the map: {:?}", map);
}