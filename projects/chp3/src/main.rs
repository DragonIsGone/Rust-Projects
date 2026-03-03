fn main() {

    let x = 5;

    let x = x+1;

    {
        let x = x*2;
        println!("Val of X inside is {x}");
    }
    println!("The val of x outside is {x}");
}
