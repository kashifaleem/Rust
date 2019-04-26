fn main() {
    /* 
    Program # 1
    let hello = "Hello, World!";
    let hello1 = hello;

    println!("{}", hello);

    let hello= String::from("Hello, World!");
    let hello1 = hello.clone();
    println!("{}", hello); */

    // Program #2
    let s1 = "Hello, World!";
    let mut s2 = String::from("Hello, World!");
    let s3 = String::from(("Hello Axiom!"));

    println!("{:p}", s1); // :p = pointer to string
    foo(s1);

    s3.push_str("World");
    println!("{}",s3);

    foo1(s3);// gives and looses ownership
    //println!("{}", s3);

    s2 = foo2(s2);
    println!("{}", s2);

   
    
}

fn foo(string: &str) { // address passing
    println!("{:p}", string);
} 
fn foo1(string: String) {
    
}
fn foo2(string: String) -> String{ // ownership taking and return
    println!("{}", string);
    string  // ownership returns
}  