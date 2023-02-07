extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        // 입력값을 저장할 공간 생성
        let mut guess = String::new();

        // read_line()은 Result를 반환함. Result는 Ok, Err variant를 가지고 있음
        // Err일 경우 expect 메소드는 프로그램이 작동을 멈추고 메세지를 출력하ㅗ록 함
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // guess는 String, secret number는 i32이므로 비교가 불가능함
        // realine에서 \n가 들어가기 때문에 trim()으로 제거
        // guess를 int로 변경. parse()는 string을 다양한 타입으로 변환할 수 있기 때문에 반드시 타입을 명시해야함
        // parse()가 실패하면 에러 메세지 출력
        let guess: u32 = match guess.trim().parse() {
            // 정수를 변환하는데 성공하면 num을 반환함
            Ok(num) => num,
            // 모든 에러는 continue를 호출
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
