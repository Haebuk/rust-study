#[derive(Debug)] // Rectangle 구조체가 Debug 프린트 가능하게 허용
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rec1 = Rectangle {
        length: 50,
        width: 30,
    };
    println!("rect1 is {:?}", rec1); // Debug print {:?} 사용
    println!("rect1 is {:#?}", rec1); // 좀 더 예쁘게 출력

    println!("area: {} square pixels", area(&rec1));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.length
}
