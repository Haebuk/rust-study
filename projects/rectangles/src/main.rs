#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };

    println!("rect1 is {:?}", rect1); // derive 어노테이션을 이용한 커스텀 타입의 동작 추가

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1) // main의 소유권 유지 (빌림)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}
