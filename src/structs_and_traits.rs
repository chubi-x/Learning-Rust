    // ##################
    // NEW STRUCT INSTANCE
    let name : String = String::from("Honda");
    let model : String = String::from("Accord");
    let car = Car {name:name, model:model, year:2002};
    car.print_name();
    car.print_model();
    car.print_year();
    println!("Can this car fly? {}",car.can_fly())
}
// ################################
//  STRUCTS (CLASSES)
struct Car{
    name: String,
    model: String,
    year: i16
}
// ##############################
// HOW TO IMPLEMENT CLASS METHODS
// ##############################
impl Car{
    fn print_name(&self){
        println!("The name of the car is {}", self.name)
    }
    fn print_model(&self){
        println!("The model of the car is {}",self.model)
    }
    fn print_year(&self){
        println!("The year of the car is {}",self.year)
    }
}
impl Vehicle for Car{
    fn can_fly (&self) -> bool {
        false
    }
}
// ##################################33
// USING TRAITS (INTERFACES) THAT CLASSES CAN IMPLEMENT
// ########################################

trait Vehicle{
    fn can_fly(&self) -> bool;
    fn is_vehicle(&self) -> bool{
        true
    }
}
