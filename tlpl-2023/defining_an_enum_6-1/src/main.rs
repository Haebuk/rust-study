// 열거형이 구조체보다 적절한 상황: 열거할 수 있는 항목 중 하나만 선택 가능할 때
// variants 중 하나만 될 수 있다라고 표현
enum IpAddrKind {
    V4,
    V6,
}
// 각 variants에 다양한 유형의 타입을 포함하는 경우
enum Message {
    Quit,                       // 연관된 데이터 없음
    Move { x: i32, y: i32 },    // 익명 구조체 포함
    Write(String),              // 단 하나의 String 포함
    ChangeColor(i32, i32, i32), // 세 개의 i32 포함
}

// 열거형도 메소드 정의 가능
impl Message {
    fn call(&self) {}
}

// 구조체를 통해 열거형 variant 저장하기
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// 값의 존재 혹은 부재를 포현할 수 있는 열거형
// <T>는 제너릭 타입파라미터, <T>가 Option 열거형의 Some variant가 어떤 타입의 데이터라도 가질 수 있다는 것을 의미
// null일 수 있는 값을 사용하려면 명시적으로 값의 타입을 Option<T>로 만들어야 함
// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    // 열거형 값
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
}
