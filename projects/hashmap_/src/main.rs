use std::collections::HashMap;

fn main() {
    // 해쉬맵 생성
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 리스트로 해쉬맵 생성
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect(); // collect 는 많은 데이터 구조로 바뀔 수 있기 때문에 해쉬맵 타입 선언, 해쉬맵 키:값 타입은 추론 가능하므로 _로 생략 가능

    // 해쉬맵과 소유권
    let field_name = String::from("Favorite color");
    let field_value = String::from("blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value); // field_name, field_value는 더 이상 유효하지 않음

    // 해쉬맵 내 값 접근
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 50);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // 반복 작업
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 해쉬맵 갱신 - 덮어쓰기
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // 해쉬맵 갱신 - 값이 없는 경우에만 삽입
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // 해쉬맵 갱신 - 예전값 기초
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // 처음 나오면 0
        *count += 1;
    }
    println!("{:?}", map);
}
