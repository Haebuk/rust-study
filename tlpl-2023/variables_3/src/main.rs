fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x); -> 에러: 기본적으로 변수는 불변성이라 바꿀 수 없음

    // 가변 선언
    let mut x = 5;
    println!("{}", x);
    x = 6;
    println!("{}", x);

    // 상수 - 상수는 무조건 불변
    // 함수의 결과값이나 시간에 따라 결정되는 값은 지정 불가
    // rust는 상수명은 대문자가 원칙
    // 타입을 반드시 명시해야 함
    const MAX_POINTS: i32 = 100_000;
    println!("{MAX_POINTS}");

    // Shadowing
    // 값의 유형을 변경하면서 동일 이름을 사용하고 싶은 경우
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("{x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}");

    // mut은 섀도잉 불가
    // let mut spaces = "   ";
    // spaces = spaces.len();
}
