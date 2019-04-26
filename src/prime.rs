use std::io;

fn main()
{

    println!("Enter the number of Prime Numbers to be generated: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = input.trim().parse().expect("Please type a number"); 
    println!("You typed {}",input);

    let mut counter =1;
    let mut num: usize = 1;
    loop {
        let res =  isPrime(num);
        if res
        {
            println!("Prime Number: {} is {}", counter, num);

            counter = counter + 1;
            if counter >  input
            {
                break;
            }
        }
        num = num+1;
        
    }
    
}

fn isPrime(num: usize) -> bool {
    
    if num == 1
    {
        return false;
    }
    else if num ==2
    {
        return true;
    }
    else{
        let mut x = 2;
        loop{
            if num % x ==0  {
                return false;
            }
            x+=1;
            if x == num {
                return true;
            }
                
        }

    }
    
}