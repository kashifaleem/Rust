use std::io;

fn main()
{
    let mut inp_value = String::new();
    
    println!("Enter a number:" );

    io::stdin().read_line(&mut inp_value).expect("Unable to read value");
    let input: i32 = inp_value.trim().parse().unwrap();//.expect("Please enter a valid value");
    
    println!("Cube of {} is {}", input, cube(input));
}

fn cube(val:i32)-> i32
{
    val*val*val
}