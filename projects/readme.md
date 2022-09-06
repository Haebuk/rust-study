# build with cargo
`cargo new hello_cargo --bin`

`cargo build`

# run
`cargo run`

# check
- 컴파일 확인, 실행파일 생성 x
- 빌드전에 체크
`cargo check`

# build release
- 배포 준비
- 기본 빌드보다 오래 걸림
- 최적화(벤치마킹)

# make library
`cargo new communicator --lib`
- main.rs 대신 lib.rs 생성
- 