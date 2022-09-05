use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // 변수 선언. 러스트 변수는 기본적으로 불변임

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // read_line 메소드가 Err 반환시 출력, Ok 반환 시 Ok가 가지고 있는 결과값 반환

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     // num 이면 반환
            Err(_) => continue, // 다른 타입이면 위로
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Greater => println!("Too big!!"),
            Ordering::Equal => {
                println!("You win!!");
                break;
            }
        }
    }
}
