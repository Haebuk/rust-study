pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of;

enum TrafficLight {
    Red,
    Yellow,
    Green,
}
// 열거형 use
use TrafficLight::{Red, Yellow};
// use TrafficLight::* // glob을 통해 모두 가져오기, 이름 충돌 가능성있으므로 아껴쓰기

fn main() {
    // a::series::of::nested_modules(); // 너무 길다

    of::nested_modules();

    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}
