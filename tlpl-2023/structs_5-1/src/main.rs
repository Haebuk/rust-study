fn main() {
    // 구조체는 구성 요소가 각자 다른 타입을 가질 수 있음
    let user1 = User {
        // 필드 순서 정의와 달라도 ok
        email: String::from("someone@example.com"),
        username: String::from("someusername21"),
        active: false,
        sign_in_count: 1,
    };

    // 특정값 읽어오기
    let email = user1.email;

    // user1의 일부값을 재사용해 user2 인스턴스 생성
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotheruser123"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    // 구조체 갱신법(struct update syntax) 사용
    // email, username은 다르고 나머지는 user1과 같음
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotheruser123"),
        ..user1
    };

    // 튜플 구조체 - 필드의 타입만 정의
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Color(0, 0, 0);
}

struct User {
    username: String, // 필드들
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 빌드 초기화 축약법(field init shorthand)
// 구조체 필드명과 매개변수 명이 같을 경우 구조체에 필드명 생략 가능
fn build_user(email: String, username: String) -> User {
    User {
        email, // email: email 대신 email만 작성
        username,
        active: true,
        sign_in_count: 1,
    }
}
