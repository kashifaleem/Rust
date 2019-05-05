  struct User {
        name: String,
        age: i8,
        active:bool,
        email: String,
    }

fn main()
{
  

    let user1 = User {
        name: String::from("Kashif"),
        age: 34,
        active: true,
        email: String::from("kashif@yahoo.com")
    };

    let user2 = User {
        .. user1
    };

    let mut user3 = function();
    user3.active = false;
    user3.email = String::from("kashif@yahoo.com");

    println!("{}",user3.email );

    fn function() -> User {
        let user = User {
            name: String::from("Kashif"),
            age: 33,
            active: true,
            email: String::from("xyz@yahoo.com"),
        };
        user
    }
}