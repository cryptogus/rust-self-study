# ch1
rustfmt를 사용하여 러스트 코드 스타일을 통일 시킬 수 있다.
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