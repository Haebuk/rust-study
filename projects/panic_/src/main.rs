use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

fn main() {
    // 복구 가능한 에러
    enum Result<T, E> {
        Ok(T),  // T는 성공일 때 타입
        Err(E), // E는 실패일 때 타입
    }

    // match로 Result 처리
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        // ref 참조(18장)
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => {
                panic!("Tried to create file but there was a problem: {:?}", e)
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    };

    // 에러 발생 시 패닉 숏컷 - unwrap
    let f = File::open("hi.txt").unwrap();

    // 에러 발생 시 패닉 숏컷 - expect: panic 메세지 지정 가능 (better)
    let f = File::open("hi.txt").expect("Failed to open hi.txt");
}

// 에러 전파
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hi.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 숏컷
fn read_user_name_from_file_short() -> Result<String, io::Error> {
    let mut f = File::open("hi.txt")?; // ?를 붙인다. 반드시 Result를 반환하는 함수내에서만 사용 가능
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
