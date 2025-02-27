fn main() {
    let mut x = 5;
    println!("the value of x is {}", x);
    x = 10;
    println!("the value of x is {}", x);
    
    println!("=========");

    let y = 5;
    println!("The value of y is {}", y);

    let y = y + 10;
    println!("The value of y is {}", y);

    let y = y * 2;
    println!("The value of y is {}", y);

    let spaces = " ";
    println!("{}", spaces);

    let spaces = 10;
    println!("{}", spaces);

    let tup: (i32, f64, u8) = (500, 6.4, 3);
    println!("{:?}", tup);
    let(a,b,c) = tup;
    println!("a: {}, b: {}, c: {}", a, b, c);
}
