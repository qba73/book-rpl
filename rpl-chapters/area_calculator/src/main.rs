fn main() {
    let rectangle1 = Rectangle{
        width: 10,
        height: 20,
    };
    println!("the rectangle is {:?}", rectangle1);

    let area = area(    &rectangle1);
    println!("the area is {}", area);

    // Print area - call the method on the struct
    println!("The area is {}", rectangle1.area());

    let rectangle2 = Rectangle{width: 5, height:2};

    let can_hold = rectangle1.can_hold(&rectangle2);
    println!("Can hold? Answer is: {}.", can_hold);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}




fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}