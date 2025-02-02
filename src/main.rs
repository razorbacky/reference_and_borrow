// Reference and Borrowing 25.02.02
// 참조와 대여

/* 이전에 작업한 Argument Ownership 내용을 간략화 하는 방법
main2.rs 에 해당 파일을 복제 해두었으니 차이점을 확인하고 참조 바람.

값의 소유권을 넘기는 대신 개체의 참조자를 넘겨주는 방법(개선).

calculate_length 에서 참조자를 매개변수로 받도록 구현한 것.
앰퍼센드(&) 기호가 참조자를 나타내고, 값의 소유권을 가져오지 않고, 해당 값을 참조할 수 있도록 한다.

즉, 앰퍼센드(&)가 '참조' 를 나타낸다.

Note :
앰퍼센드(&)를 이용한 참조의 반대는 역참조(dereferencing)이라 한다.
역참조 기호는 애스터리크(*)이다.
*/

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    // 기존 변수 선언부에 있던 튜플 코드가 없다.
    // calculate_length 함수에 s1 대신 &s1 을 전달한다.

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // 함수 정의에 String 대신 &String을 사용한다.

    s.len()
    // 함수 반환 값에 위치하던 튜플 코드가 없다.
}
