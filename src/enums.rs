fn main(){
    let a : MyEnum = MyEnum :: A;
    let b : MyEnum = MyEnum :: B (4000);
    let c : MyEnum = MyEnum :: C{x: 400, y:49999};
    // printing the enum fields
    println!("Lets print the enum values! \n 
     {:?} for A, {:?} for B, and {:?} for C!",a,b,c);
    //printing the values of the enum fields
    if let MyEnum ::B(val) =b {
        println!("this is the value of the B field: {} ", val);
    }
    if let MyEnum ::C{x,y} =c {
        println!("these are the value of the C field: {}, {} ", x,y);
    }
}

#[derive (Debug)]
enum MyEnum {
    A,
    B (i32),
    C {x: i16, y: i64}
}