# ch1

## 주석 관련 추가 내용
컴파일러가 변수 앞에 _가 붙으면 무시해버림  \
변수 이름으로 한글도 가능함

## rustfmt
rustfmt를 사용하여 러스트 코드 스타일을 통일 시킬 수 있다.  \
rustfmt 도구를 설치.
```bash
$ rustup component add rustfmt
```
코드 스타일 변경
```bash
$ rustfmt hello_world.rs
```
코드 스타일 변경 전
```rs
fn main()
{

  printfln!();
```
코드 스타일 변경 후
```rs
fn main() {
    printfln!();
```
참고로 golang은 코드 스타일 변경 후 와 같이 `{`를 사용하지 않으면 오류를 낸다. 그래서 나는 `c/c++` 도 코드 스타일을 현재 저런식으로 통일해서 사용하고 있다.
