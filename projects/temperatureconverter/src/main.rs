use std::io;

fn main(){
    println!("Select a function");
    println!("1. Celsius to farenheight");
    println!("2. Farenheight to Celsius");
    
    let mut choice: String = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("failed to read line");

    match choice.trim() {
        "1" => convert_c_to_f(),
        "2" => convert_f_to_c(),
        _ => println!("Invalid Selection"),
    }
}

fn convert_c_to_f(){
    //convert cel to far (cringe)
    println!("Enter Celcius amount: ");

    let mut input = String::new();
    io::stdin() 
        .read_line(&mut input)
        .expect("Failed to Read Line");

    let input: f32 = match input.trim().parse::<f32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a number run again");
            return;
        },
    }; 
    let result = input * 1.8 + 32.0;
    println!("Result: {result}°F");
}

fn convert_f_to_c(){
    println!("Far to celcius: ");

    let mut input = String::new();
    io::stdin() 
        .read_line(&mut input)
        .expect("Failed to Read Line");

    let input: f32 = match input.trim().parse::<f32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a number run again");
            return;
        },
    }; 
    let result = (input - 32.0) / 1.8;
    println!("Result: {result}°C");
}