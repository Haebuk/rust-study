#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

// 메소드
// Rectangle 내용 안에 함수 정의 위해 impl 블록 시작
// self 타입이 Rectangle 타입임
// 소유권을 원하지 않고 구조체 내의 데이터를 읽기만 하고 싶으므로 빌린다.

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    // 또다른 Rectangle이 self안에 들어오는지 확인하는 메소드
    fn can_hold(&self, another: &Rectangle) -> bool {
        another.length < self.length && another.width < self.width
    }

    // 연관 함수: self 파라미터 갖지 않아도 impl 내 구현
    // 새로운 구조체의 인스턴스를 반환하는 생성자로 주로 사용
    fn square(size: u32) -> Rectangle {
        Rectangle {
            length: size,
            width: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };
    let rect2 = Rectangle {
        length: 40,
        width: 10,
    };
    let rect3 = Rectangle {
        length: 45,
        width: 60,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(10);
    println!("Area of Square: {}", sq.area());
}
