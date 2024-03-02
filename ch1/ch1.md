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
Rust에서 변수는 기본적으로 불변(immutable)으로 선언되기 때문에 한 번 값을 할당하면 값을 변경할 수 없다.  \
```rs
fn main() {
    let x = 10;  // 불변 변수 x를 선언하고 10으로 초기화함
    x = 20;      // 에러: x는 불변 변수이므로 재할당할 수 없음
}
```
변수를 가변(mutable)으로 선언하여 값을 변경할 수 있도록 할 수 있다.
```rs
fn main() {
    let mut x = 10;  // 가변 변수 x를 선언하고 10으로 초기화함
    x = 20;          // 이제 x는 가변이므로 값을 변경할 수 있음
    println!("x: {}", x);
}
```
소스 코드 내의 주석을 확인하며 컴파일 해본다.
```bash
$ rustc variable.rs
```
러스트는 기본적으로 다른 type 끼리의 변수들로 연산을 진행하면 오류가 난다. 이를 해결하려면 casting(형 변환)을 해주면 된다.  \
c언어에서는 개발자가 직접 변수의 앞에 `(int)`등 형 변환을 해주거나 컴파일러가 알아서 형 변환을 진행하여 연산을 진행했었다.  \
러스트에서는 `as` 라는 키워드로 진행한다.
```rs
// casting = simple type change using 'as'
let first_num: u16 = 8;
let second_num: u8 = 16;
let third_num = first_num + second_num as u16;
```
char(charicter) 형
```rs
// c언어와 동일하게 단일 문자 하나라면 '', 문자열이라면 ""로 감싸야한다.
// "hello" -> O, 'hello' -> X, 'a' -> O
let space = ' ';
let other_language = 'ㄱ'; // thanks to unicode, 기본적으론 유니코드다.
let clover = '🍀'; // Emojis are chars too
char = 4 bytes

let ascii = 'a' as u8; // 명확하게 ascii 코드로 쓰는 방법
let not_ascii = 'ㄱ' as u8; // 유니코드를 ascii 코드 크기로 잘 못 캐스팅 하면 쓰는 방법
let unicode = 'ㄱ' as u32; // 유니코드
println!("ascii: {}", ascii);
println!("not_ascii: {}", not_ascii);
println!("unicode: {}", unicode);

// string에서 .len()은 길이 값이 아니라 전체 바이트 값을 나타낸다. 유니코드에서 a,b,c 등 알파벳은 1바이트, ㄱ은 3바이트만 사용하기에 그 숫자가 표현됨.
println!("a len: {}", "a".len());
println!("ㄱ len: {}", "ㄱ".len());
println!("ㄱㄴㄷㄻ: {}", "ㄱㄴㄷㄻ".len());
```
마지막 정리
```rs
let num = 10u8; // u8 크기인 10을 나타냄
let num2 = 10_u8; // u8 크기인 10을 나타냄
let num3 = 10________________u8; // u8 크기인 10을 나타냄, _ 가 얼마나 있던 컴파일러는 무시
let num4 = 1_000_000_000u32; // 따라서 이런 것 도 가능

// 소수점 자료형 floating point
f32 / f64 두 개
let floating = 9.6; // f64
floating as i32; // 찍어보면 9로 출력됨. 이 부분은 c언어와 같다.
```