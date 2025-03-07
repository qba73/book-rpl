fn main() {
    let rectangle1 = Rectangle{
        width: 10,
        height: 20,
    };
    println!("the rectangle is {:?}", rectangle1);

    let area = area(    &rectangle1);
    println!("the area is {}", area);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}