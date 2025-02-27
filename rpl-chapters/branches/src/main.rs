fn main() {
    println!("Branching time!");

    let number = 4;

    if number < 5 {
        println!("dupa")
    } else {
        println!("alio!")
    }

    // looping

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 5 {
            break counter*3;
        }
    };

    println!("Counter in the loop is: {}", result);

    // while loop

    let mut number = 2;

    while number != -5 {
        println!("{}!", number);
        number -= 1;
    }

    println!("{}", number);

    // for loops
    println!("========= for loops 1 ========");

    let a = [1,3,4,2,1];
    for element in a.iter() {
        println!("{}", element);
    }

    println!("========= for loops 2 ========");
    for number in (1..10).rev() {
        println!("{}", number);
    }
    println!("========= for loops 3 ========");
    
    for number in (1..10).enumerate() {
        println!("{}", number.1);
    }

}

