#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rectl = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rectl)
    );

    println!("rectl is {:?}", rectl);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
