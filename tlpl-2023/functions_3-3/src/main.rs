fn main() {
    another_function(5, 6);

    // 구문과 표현식
    // 구문은 값 반환x, 표현식은 값 반환o
    let y = 6; // 구문, 6은 6이란 표현식

    let y = {
        let x = 3;
        x + 1 // no semicolon. 표현식은 세미콜론 사용하지 않음
    };

    println!("The y: {y}");

    let x = five();
    println!("x, {x}")
}

// rust는 snake case를 사용
// main 뒤에도 정의 가능
fn another_function(x: i32, y: i32) {
    println!("Another function, {x}, {y}")
}

fn five() -> i32 {
    5 // 암묵적으로 마지막 표현식을 반환.
}