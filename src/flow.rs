    // FOR LOOP
    for _x in _arr{
        // println!("{x}")
    }
    switch();
    println!("\nrunning a match {:?}", switch_num);
    WHILE LOOP 
    let mut count:i8 = 0;
    while count <=5{
        println!("The count is {count}");
        count+=1
    }
    // Switch function
    pub fn switch (){
        let i = 4;
        return  match i {
                0 => println!("0"),
                1 | 2 => println!("1 or 2"),
                3..=4 => println!("3,4"),
                _ => println!("default")
        }
    }
    
    
    