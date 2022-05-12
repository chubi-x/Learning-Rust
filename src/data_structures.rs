    //ARRAYS
    let num:i8 = 4;
    let _arr: [i8;5] = [num;5];
    let _other_arr: [i8;4] = [0,2,4,5];
    println!("This array, {:?} is populated with {} numbers", _other_arr, _other_arr.len());
    // ###################################
    // ############################################
    //TUPLE
    let tuple: (i8, bool, &str) = (1,false,"hello world");
    //accessing tuple elements
    println!("In this tuple, {:?}, the first element is {}", tuple, tuple.0);
    //destructing the tuple
    let (a,b,c) = tuple;
    println!("Destructuring this tuple, its elements are {}, {}, and {}", a,b,c);
    
    
    //SLICES
    let arr = [1,2,3,4,5];
    let slice = &arr[0 .. 4];
    println!("The first element of the slice {:?}, is {}", slice,slice[0]);
    
