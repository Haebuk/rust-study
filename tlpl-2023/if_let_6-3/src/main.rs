fn main() {
    // 하나의 패턴만 일치시키고 싶은 경우에는 match보다 if let
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (), // 너무 불필요한 코드
    }

    if let Some(3) = some_u8_value {
        println!("three")
    } // 간결함을 얻을지, 전수조사를 얻을지 선택

    if let Some(3) = some_u8_value {
        println!("three")
    } else {
        println!("not three")
    }
}
