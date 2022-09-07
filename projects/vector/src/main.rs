fn main() {
    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

    // let does_not_exist = &v[100]; //panic
    let does_not_exist = v.get(100); //None

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6); //

    // 반복 처리
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // 반복 처리 - 변형
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // 열거형 사용하여 여러 타입 지정
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
