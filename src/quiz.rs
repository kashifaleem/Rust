/* fn main () {
let mut x = 5;
take_it(x); 
x=4;
println!("{}, ", x);
}
fn take_it(mut xyz: i32) {
    xyz = 6;
println!("{}, ", xyz);
}
 */

/* fn main() {
    let mut s = String::from("hello");
    change(&mut s);
println!("{}",s);
}
fn change(some_string: &mut String) {
    println!("{:p}",some_string );
    some_string.push_str(", world");
    println!("{:p}",some_string );
} */

/* fn main() {
let mut s = String::from("hello"); 
let r1 = &mut s; 
let r2 = &mut s; 
println!("{}, {}", r2, r2);
} */

fn main() {
    let mut s = String::from("hello"); 
    {
        let r1 = &mut s; 
        println!("{}", r1 );
    }
    let r2 = s; 
    //r2.push_str("world");
    let r3= s;
    println!("{} {}",  r2, r3);
}
