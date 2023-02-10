fn main() {
    let s = String::from("Hello world");

    // start 포함 end 미포함
    let hello = &s[0..5]; // 처음이 0이면 생략 가능
    let world = &s[6..11]; // 마지막 바이트를 포함한다면 끝 숫자 생략 가능
    let slice = &s[..]; //전체 스트링의 슬라이스 - 양쪽 모두 생략 가능

    // 스트링 리터럴은 슬라이스임
    let s = "Hello, world!"; // s의 타입은 &str: 불변 참조자

    let my_string = String::from("hello world");
    // String을 갖고 있으면 String의 전체 슬라이스를 넘길 수 있음
    let word = first_word(&my_string[..]);
    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[..]);
    // 스트링 리터럴은 또한 스트링 슬라이스도 되기 때문에 슬라이스 문법 없이도 작동
    let word = first_word(my_string_literal);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}

// 첫번째 단어를 반환하는 함수
// 공백이 발견하면 해당 인덱스 전까지 반환
// 공백이 없으면 전체 단어 반환 (한 단어)
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
