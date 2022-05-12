// FUNCTIONS
fn main(){
    println!("{}",is_even(3));
}
pub fn is_even(num: u8) -> bool{
    let digit = num %2;
    digit==0 //return statements don't have semicolon after them
}
