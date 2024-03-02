fn main() {
    let my_number = 100; // 변수의 자료형을 정의 안해두면 일단 시스템에 따라 i32
    let my_number2: u8 = 100;

    // type inference
    let third_number = my_number + my_number2; // 보통 u8 + i32는 절대 안된다고 컴파일 오류가 뜸, 하지만 my_number의 타입을 정해두지 않았기 때문에 오류 X
    let my_number3: i32 = 100;
    // let third_number2 = my_number2 + my_number3; // 컴파일 에러가 뜸. -> cannot add `i32` to `u8`

    // casting = simple type change using 'as'
    let first_num: u16 = 8;
    let second_num: u8 = 16;
    let third_num = first_num + second_num as u16;

    let ascii = 'a' as u8; // 명확하게 ascii 코드로 쓰는 방법
    let not_ascii = 'ㄱ' as u8; // 유니코드를 ascii 코드 크기로 잘 못 캐스팅 하면 쓰는 방법
    let unicode = 'ㄱ' as u32; // 유니코드
    println!("ascii: {}", ascii);
    println!("not_ascii: {}", not_ascii);
    println!("unicode: {}", unicode);

    // string에서 .len()은 길이 값이 아니라 바이트 값을 나타낸다.
    println!("a len: {}", "a".len());
    println!("ㄱ len: {}", "ㄱ".len());
    println!("ㄱㄴㄷㄻ: {}", "ㄱㄴㄷㄻ".len());
    println!("ʊ: {}", "ʊ".len()); // 독일어
    println!("ʊʊʊʊ: {}", "ʊʊʊʊ".len());
}
