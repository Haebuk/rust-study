use std::vec;

fn main() {
    // i32 타입의 값을 가질 수 있는 새 벡터 생성
    // 아무값도 넣지 않을 경우 타입 명시해야함.
    let v: Vec<i32> = Vec::new();

    // 초기 편의성 -> 초기 값을 통한 타입 유추
    let v = vec![1, 2, 3];

    // 벡터 갱신하기
    let mut v = Vec::new(); // 밑에 값을 집어넣음으로써 타입 유추
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // 벡터의 요소 읽기: 두 가지 방법
    // 첫 번째 방법은 벡터를 넘어서는 요소에 접근하면 panic
    // 두 번째 방법은 None을 반환 -> Some(&element), None에 대해 다루는 로직 추가 필요
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

    // 유효하지 않은 참조자
    // 같은 스코프 내에서 가변 참조자와 불변 참조자 가질 수 없음
    // 첫번째 요소에 대한 참조자가 벡터 끝에 대한 변경을 걱정하는 이유:
    // 새로운 요소를 벡터 끝에 추가하는 것은 새로 메모리를 할당해 예전 요소를 새 공간에 복사하는 일을 필요로 할 수 있는데,
    // 이는 벡터가 모든 요소들을 붙여 저장할 공간이 충분치 않는 환경에서 일어날 수 있음
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6); // 에러난다는데 왜 되지..

    // 벡터 내 값들에 대한 반복 처리
    let v = vec![100, 32, 54];
    for i in &v {
        println!("{i}");
    }

    // 벡터 내 요소에 대해 가변 참조자로 반복
    // 역참조 연산자로 값 얻기
    let mut v = vec![100, 32, 54];
    for i in &mut v {
        *i += 50;
    }

    // 열거형을 사용해 여러 타입 저장하기
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];
}
