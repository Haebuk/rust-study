fn main() {
    // 스트링 리터럴로부터 String 생성
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    // 정확히 같은 일
    let s = String::from("initial contents");

    // push_str과 push를 이용해 스트링 추가
    let mut s = String::from("foo");
    s.push_str("bar"); // foobar

    // push는 한 글자
    let mut s = String::from("lo");
    s.push('l');

    // + 연산자나 format! 매크로를 이용한 접합
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // s1은 더한 후 더이상 유효하지 않고
    // s2는 + 연산자를 사용했을 때 호출되는 함수 시그니처의 타입과 맞추기 위해 참조자 사용
    // fn add(self, s: &str) -> String {}
    // &String은 &str로 강제될 수 있기 때문에 문제 없음
    let s3 = s1 + &s2; //

    // 더 복잡한 스트링 조합
    let s1 = String::from("tic");
    let s1 = String::from("tac");
    let s1 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3); // tic-tac-toe

    // 스트링에 인덱싱 시도 -> 에러!
    let s1 = String::from("hello");
    // let h = s1[0];

    // 스트링 슬라이싱
    let hello = "Здравствуйте";
    let s = hello[0..4];
}
