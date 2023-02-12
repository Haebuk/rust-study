#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // _ 변경자(placeholder)
    // 모든 가능한 값을 나열하고 싶지 않을 때 사용
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        _ => (), // 1,3 을 제외하고 아무일도 일어나지 않음
    }
}

// match를 사용하면 if와 달리 bool 뿐만 아니라 어떤 타입이든 반환이 가능
// 결과 값을 순차적으로 비교
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            // 여러 줄의 코드를 실행시키고 싶은 경우
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// x가 존재하면 x+1, 없으면 None
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
