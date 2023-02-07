fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // with explicit type annotation

    // char 타입은 작은 따옴표
    // 큰 따옴표는 string
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // 튜플 - 여러 타입을 묶어 복합 타입으로 표현 가능
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // 구조해체 - 튜플의 값 밖으로 꺼내기
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{y}"); // 6.4

    // 인덱싱
    let x = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // 배열 - 모든 요소는 같은 타입이어야 함
    // 한번 선언되면 길이가 변하지 않음
    // 스택에 할당하는 것을 원할 때 사용 (고정적인 요소 개수를 원할때)
    // 가변적인 것을 원하면 벡터를 사용
    let a = [1, 2, 3, 4, 5];

    // 배열 요소 접근
    let first = a[0];
    let second = a[1];

    // 배열 밖의 요소에 접근할 때는 컴파일시에는 에러 발생하지 않음
    // 결과 실행 중 에러 발생
    // 색인을 사용해 요소에 접근할 때 Rust는 먼저 색인이 배열 길이보다 작은지 확인
    let element = a[10];
}
