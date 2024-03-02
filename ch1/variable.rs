fn main() {
    let my_number = 100; // 변수의 자료형을 정의 안해두면 일단 시스템에 따라 i32
    let my_number2 : u8 = 100;

    // type inference
    let third_number = my_number + my_number2; // 보통 u8 + i32는 절대 안된다고 컴파일 오류가 뜸, 하지만 my_number의 타입을 정해두지 않았기 때문에 오류 X
    let my_number3 : i32 = 100;
    third_number = my_number2 + my_number2; // 컴파일 에러가 뜸. -> cannot assign twice to immutable variable
}