fn main() {
    let s1 = gives_ownersip();

    println!("{s1}");

    let s2 = String::from("Alio!"); // s2 comes into scope
    
    let s3 = takes_and_gives(s2); // s2 is moved into the func; the func moves returned value into s3
    println!("{s3}");

    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn gives_ownersip() -> String {
    let some_string = String::from("hello");
    some_string // it's returned and moves out to the scope of the calling function
}

fn takes_and_gives(a_string: String) -> String {
    a_string
}


fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}