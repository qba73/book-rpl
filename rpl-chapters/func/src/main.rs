fn main() {
    another_function(3);

    let x = 5;
    println!("{x}");

    let y = {
        let x = 3;
        x + 1
    };
    println!("{y}");

    let x = five();
    println!("{x}");

    let y = plus_one(1);
    println!("plus one: {}", y)

}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(x: i32) {
    println!("This is value {}", x);
}