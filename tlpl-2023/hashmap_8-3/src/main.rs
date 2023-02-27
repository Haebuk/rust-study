fn main() {
    use std::collections::HashMap;

    // 새로운 해쉬맵을 생성하여 몇 개의 키와 값을 집어넣기
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 팀의 리스트와 점수의 리스트로부터 해쉬맵 생성하기
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // 키와 값이 삽입되는 순간 이들이 해쉬맵의 소유가 되는 것을 보여주는 예
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name과 field_value는 이 지점부터 유효하지 않음

    // 해쉬맵 내에 저장된 블루 팀의 점수 접근하기
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // for 루프를 사용해 각각으 키/값 쌍 가져오기
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // 해쉬맵 갱신하기 - 새값으로 대체할 지, 없는 경우에만 추가할지 등등

    // 값을 덮어쓰기
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores); // {"Blue": 25}

    // 키에 할당된 값이 없는 경우에만 삽입하기
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores); // {"Yellow": 50, "Blue": 10}

    // 예전 값을 기초로 값을 갱신하기
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map); // {"wonderful": 1, "world": 2, "hello": 1} 
}
