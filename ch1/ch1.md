# hello_world.rs

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
# variable.rs

c/c++ 과 마찬가지로 러스트도 변수의 크기, 부호(+/-) 여부를 결정할 수 있다.

마치 c언어의 stdint.h 나 아니면 `u8, u16, u32, u64, s8, s16, s32, s64` 같은 자료형을 볼 수 있는데 이는 각각 `unsinged 8bit, 16bit, 32bit, 64bit` 그리고 `singed 8bit, 16bit, 32bit, 64bit`이다.

각 OS(architecture) 마다 재정의된 내용이며, 이를 사용하는 목적은 int, float, double 등의 자료형은 architecture 마다 크기가 틀려질 수 있기 때문에 이를 방지하고 원하는 크기의 자료형을 사용하기 위해 명시적으로 쓰인다.

러스트의 자료형
```rs
+ plus sign
- minus sign

i8, i16, i32, i64, i128, isize -> i는 interger(정확히 signed interger)
u8, u16, u32, u64, u128, usize -> u는 unsinged integer 를 의미한다.

isize, usize는 computer architecture에 따른 값이다.
예를 들어 32비트 시스템에서는 isizr 값은 i32, 64비트 시스템에서는 isize 값은 i64와 같다.
```

소스 코드 내의 주석을 확인하며 컴파일 해본다.
```bash
$ rustc variable.rs
```

char(charicter) 형
```rs
// c언어와 동일하게 단일 문자 하나라면 '', 문자열이라면 ""로 감싸야한다.
// "hello" -> O, 'hello' -> X, 'a' -> O
```