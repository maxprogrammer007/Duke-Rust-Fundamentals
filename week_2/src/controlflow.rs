fn main() {
    let proceed = false;
    if proceed {
        println!("Proceeding");
    } else {
        println!("Not proceeding");
    }

    let height = 190;
    if height < 180 {
        println!("Tall");
    } else if height > 170 {
        println!("Average");
    } else {
        println!("Short");
    }
    let age = 19;
    if age < 12{
        println!("Child");
    }
    else if age > 13 && age < 19{
        println!("Teenager");
    } 
    else{
        println!("Adult");
    }

}
