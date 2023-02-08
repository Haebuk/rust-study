/* 소유권 규칙
1. 러스트의 모든 값은 각각 오너라고 불리는 변수를 가지고 있다.
2. 한번에 딱 하나의 오너만 존재할 수 있다.
3. 오너가 스코프 밖으로 나올 때 버려진다.
*/

fn main() {
    // s는 유효하지 않음
    // 스트링 리터럴 s (불변)
    let s = "hello"; // s는 여기부터 유효

    let mut s = String::from("hello");

    s.push_str(" world!"); // s 에 world!를 붙인다.
    println!("{s}");

    /*
    스트링 리터럴은 불변하는 것을 대가로 빠름
    String 타입은 변경 가능하고 커질 수 있는 텍스트를 위해 만들어진 것
    String::from()으로 필요한 만큼의 메모리를 요청하고
    사용이 끝났을 때 메모리를 반납한다. (스코프를 벗어났을 때)
    */

    // 변수 값 이동
    let x = 5;
    let y = x; // x의 복사본 y -> Ok

    let s1 = String::from("hello");
    let s2 = s1; // 위 x, y 와 다르게 동작함

    /*
    String은 포인터, 길이, 용량으로 이루어져 있고 포인터는 데이터가 있는 스택의 위치를 가르킴
    s1의 데이터를 s2가 그대로 복사하면 데이터가 커질 수록 런타임도 길어지는 문제가 있음
    그래서 s2가 복사될 때 데이터는 복사하지 않고 s1의 포인터, 길이, 용량만 복사함
    즉 s1과 s2는 같은 위치를 가르킴
    러스트는 스코프를 벗어날 때 메모리를 해제한다고 했음, s1과 s2가 스코프를 벗어날 때 같은 메모리를 두 번 해제하므로 메모리 해제 오류가 발생한다.
    이를 해결하기 위해 러스트는 위처럼 복사를 시도하는 대신 복사될 때 s1은 더이상 필요하지 않다 가정함 (즉 스코프 밖으로 벗어나도 s1은 아무일도 일어나지 않음)

    정수형은 스택에 저장되기 때문에 복사본을 빠르게 만들 수 있어 그냥 문제없이 수행됨
    이렇게 스택에 저장할 수 있는 타입은 Copy 트레잇을 가지고 있음
    */

    // println!("{s1}"); // move occurs

    // 정말 깊이 복사하고 싶다면?
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("{s1}, {s2}")

    let s = String::from("hello");  // s가 스코프 안으로 들어왔습니다.

    takes_ownership(s); // s의 값이 함수 안으로 이동했습니다...
                                    // ... 그리고 이제 더이상 유효하지 않습니다.
    let x = 5;                 // x가 스코프 안으로 들어왔습니다.

    makes_copy(x);     // x가 함수 안으로 이동했습니다만,
                                    // i32는 Copy가 되므로, x를 이후에 계속
                                    // 사용해도 됩니다.


    // 반환 값과 스코프
    let s1 = gives_ownership();         // gives_ownership은 반환값을 s1에게
    // 이동시킵니다.

    let s2 = String::from("hello");     // s2가 스코프 안에 들어왔습니다.

    let s3 = takes_and_gives_back(s2);  // s2는 takes_and_gives_back 안으로
                                    // 이동되었고, 이 함수가 반환값을 s3으로도
                                    // 이동시켰습니다.
} // 스코프가 끝나면 s는 버려짐

fn takes_ownership(some_string: String) {
    // some_string이 스코프 안으로 들어왔습니다.
    println!("{}", some_string);
} // 여기서 some_string이 스코프 밖으로 벗어났고 `drop`이 호출됩니다. 메모리는
  // 해제되었습니다.

fn makes_copy(some_integer: i32) {
    // some_integer이 스코프 안으로 들어왔습니다.
    println!("{}", some_integer);
} // 여기서 some_integer가 스코프 밖으로 벗어났습니다. 별다른 일은 발생하지 않습니다.


fn gives_ownership() -> String {             // gives_ownership 함수가 반환 값을 호출한 쪽으로 이동시킵니다.
    let some_string = String::from("hello"); // some_string이 스코프 안에 들어왔습니다.
    some_string                              // some_string이 반환되고, 호출한 쪽의 함수로 이동됩니다.
}

// takes_and_gives_back 함수는 String을 하나 받아서 다른 하나를 반환합니다.
fn takes_and_gives_back(a_string: String) -> String { // a_string이 스코프 안으로 들어왔습니다.
    a_string  // a_string은 반환되고, 호출한 쪽의 함수로 이동됩니다.
}