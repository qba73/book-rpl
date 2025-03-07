fn main() {
    let my_string = String::from("hello world");

    let word = first_word_ver_2(&my_string[..]);
    println!("{:?}", word);

    let my_string_literal = "hello world";

    let word = first_word_ver_2(&my_string_literal[..]);
    println!("{:?}", word);

    // this works because string literal 
    let word = first_word_ver_2(my_string_literal);
    println!("{:?}", word);


    // === PART 2 ===
    let mut s = String::from("hello world!");
    let wrd = first_word(&s);
    //s.clear(); // this results in error, cannot borrow `s` as mutable as it's already borrowed as immutable!
    println!("the first word is: {}", wrd);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }
    &s[..]
}

fn first_word_ver_2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
