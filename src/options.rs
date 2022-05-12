fn divide (dividend: i32, divisor: i32) -> Option<i32>{
    if dividend%divisor != 0 {
        None
    } else{
        Some (dividend/divisor)
    }
}
fn main(){
    let res1: Option<i32> = divide(12,2);
    let res2: Option<i32> = divide(2,12);
    // printing the some and unwrapping it for both divide function calls
    println!("The Option is ,{:?} and the unwrapped version is {}",res1, res1.unwrap());
    // For res 2
    println!("The Option is ,{:?}",res2);
}