fn main() {
    let s = String::from("home");

    let len = calculate_length(&s);
    println!("size is {}, var is {}", len, s);

    // =========

    let mut s1 = String::from("automation");

    {
        let t1 = &mut s1;
        println!("{t1}")
    } // t1 goes out  of scope here, se we can make a new reference with no problem!

    let t2 = &mut s1;
    println!("{t2}");

    // Dangling reference

    let ref_to_nothing = dangle();
    println!("{ref_to_nothing}");


}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// This won't work -> dangling reference!
// fn dangle() -> &String {
//     let s = String::from("danling!");
//     &s
// }

// This will work as the var changes ownership
fn dangle() -> String {
    let s = String::from("alio alio");
    s
}