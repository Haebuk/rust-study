fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // rust는 number를 boolean으로 자동 변환하지 않음
    // if number {
    //     println!("number was three");
    // }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        // 6이 2로 나눠 떨어져도 첫번째 해당하는 블록인 3을 찾아 나갔기 때문
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    // if문은 표현식이므로 let 구문의 우측에 사용할 수 있음
    let number = if condition { 5 } else { 6 };
    println!("number: {number}");

    // if 문은 하나의 타입만 가질 수 있음 -> 변수가 하나의 타입만을 가져야 하기 때문
    // let number = if condition { 5} else {"six"};

    // loop
    // loop {
    //     println!("my loop");
    // }

    // while
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number = number - 1
    }
    println!("LIFTOFF!");

    // for
    // 안전하고 간결 -> 주로 사용
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {element}");
    }

    // range
    // rev: 거꾸로 -> 3,2,1
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!");
}
