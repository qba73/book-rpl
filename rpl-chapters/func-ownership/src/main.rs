fn main() {

    let s = String::from("hello");

    takes_ownership(s);

    // println!("{s}"); doesn't compile - borrow of moved value
 
    let x = 5;

    makes_copy(x); // x is Copy, safe to use it after the func call

    println!("{}", x);
}


fn takes_ownership(name: String) {
    println!("{}", name);
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
}