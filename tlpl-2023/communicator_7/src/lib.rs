mod client; // contents of client.rs
            // 러스트는 기본적으로 src/lib.rs만 쳐다보기 때문에 명시적으로 다른 파일 추가
mod network;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// foo라는 모듈이 서브모듈을 가지고 있지 않다면, foo.rs에 foo에 대한 선언을 해야 함
// foo라는 모듈이 서브모듈을 가지고 있다면, foo/mod.rs에 넣어야 함