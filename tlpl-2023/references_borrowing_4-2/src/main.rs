fn main() {
    // 참조자(&)를 정의하면 소유권을 넘기는 대신 참조할 수 있도록 한다.
    let s1 = String::from("Hello!");
    let len = calculate_length(&s1);
    println!("{s1}, {len}");

    // 빌린 무언가를 고치려고 시도한다면 오류가 발생함
    // 참조자도 변수와 마찬가지로 불변임
    // 가변 참조자를 생성하면 변경 가능함
    let mut s = String::from("Hello");
    change(&mut s);
    println!("{s}");

    // data race를 방지하기 위해 특정 데이터에 가변 참조자는 단 하나만 만들 수 있음
    let r = &s;
    let r1 = &mut s;
    let r2 = &mut s;
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s는 스코프밖으로 벗어나지만, 가르키고 있는 값의 소유권이 없기 때문에 아무 일도 발생하지 않음

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
