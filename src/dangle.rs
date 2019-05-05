/* fn main(){
    let  reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);
}

fn dangle() -> &'static String {
    let s = String::from("hello");
    &s
} */

fn main()  {
    let pointer_s = dangle();
    
    const CHECK :u32 = 100;
    
    println!("{} ", pointer_s);
    let r1 = &pointer_s;
    let r2 = &pointer_s;

    println!("{}, {}", r1,r2);

    let r1 = 10;
    println!("oct {}",r1);

    println!("{}",CHECK);

    let a= println!("hi" );

    //let s = "Hello";
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("{}",r1);
        
        let r2 =&mut s;
        println!("{}",r2);
        
    }
    
}
fn dangle() -> String {
   
    let zia = String::from("Axiom");
    zia 
    }