fn main() {
    // 빈 스트링
    let mut s = String::new();

    // 초기값 (to_string)
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();

    // 다른 방법 (String::from)
    let s = String::from("initial contents");

    // 스트링 추가 (push_str)
    let mut s = String::from("foo");
    s.push_str("bar"); //foobar

    // 한 글자 추가 (push)
    let mut s = String::from("lo");
    s.push('l'); // lol

    // 결합
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1은 이동되어 더이상 사용 불가

    // 더 많이 결합
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3); // easy

    // 스트링 반복
    for c in "hello".chars() {
        println!("{}", c);
    }
}
