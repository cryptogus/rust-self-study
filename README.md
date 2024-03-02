# rust-self-study
Thanks to  \
https://www.youtube.com/watch?v=W9DO6m8JSSs&list=PLfllocyHVgsSJf1zO6k6o3SX2mbZjAqYE
https://doc.rust-kr.org/
https://google.github.io/comprehensive-rust/ko/

Ubuntu 22.04 LTS 환경에서 진행하였음. vscode사용자라면 rust-analyzer라는 Rust language support for Visual Studio Code 확장을 설치하는 것도 좋다.
## install rust
https://www.rust-lang.org/tools/install
```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
## 설치 확인(재부팅 필요)
```bash
$ rustc --version
```
## 예제 파일 컴파일
[ch1](ch1/ch1.md)
```bash
$ cd ch1
$ rustc hello_world.rs
# 주석 사용법은 c/c++ 과 같고 추가적으로 /// 문서화 라는게 있다.
# rustc는 gcc, g++, clang 등과 같은 기능을 수행
```