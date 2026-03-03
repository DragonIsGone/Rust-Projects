use std::io;

fn main(){
    //convert cel to far (cringe)

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
    convert(input);
}

fn convert(input: f32) {
    let result = input * 1.8 + 32.0;
    println!("{result}");
}