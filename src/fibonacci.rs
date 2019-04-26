use std::io;

fn main()
{
    let mut input= String::new();
    
    println!("Enter the number of elements to be generated in Fibonacci Series:" );
    io::stdin().read_line(&mut input).expect("Unable to read");

    let input:usize = match input.trim().parse(){
        Ok(input) => input,
        Err(_) => panic!("Please input proper integer value.")
    };

   

    fibonacci(input);

}
fn fibonacci(numbers: usize){

    let mut fib= vec![0; numbers];
    fib[0] = 0;
    fib[1] = 1;

    for number in 2..numbers {
        fib[number] = fib[number-1] + fib[number-2];
    }
    
    println!("Output" );
    for i in 0..numbers {
        println!("{}", fib[i] );
    }
}

