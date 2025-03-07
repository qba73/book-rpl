fn main() {
    let user1 = make_user("Bolek".to_string(), "bolek@wp.pl".to_string());
    println!("{:?}", user1);

    let user2 = User{
        username: String::from("Bolek"),
        email: String::from("bolek@wp.pl"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    println!("{:?}", user2);
    println!("{}, {}", user1.username, user1.email);

    // Tuple structs:
    //let color1 = Color(0, 0, 0);
    let point = Point2D(2,3);
    println!("{}, {}", point.0, point.1);
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn make_user(username: String, email: String) -> User {
    User{
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

// tuple structs
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point2D(i32, i32);
struct Point3D(i32, i32, i32);
