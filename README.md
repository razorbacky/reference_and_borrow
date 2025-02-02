[Rust] Reference and Borrowing
===

**이전의 Argument Ownership 25.01.30 에서 진행한 내용의 개선 버전이므로, 해당 내용을 참조한다.**

값의 소유권을 넘겨주는 대신 개체의 참조자를 넘겨주고 소유권을 유지하는 방법.

앰퍼센드(&) 기호가 참조자를 나타내며, 값의 소유권을 가져오지 않고, 해당 값을 참조할 수 있도록 한다.

앰퍼센드(&)를 이용한 참조의 반대는 **역참조(dereferencing)** 이라 하며,

역참조 기호는 **애스터리크(*)** 이다.


**기존 코드**
```rust
fn main() {
    let s1 = String::from("Hello");

    let (s2, len) = calculate_length(s1.clone());

    println!("The length of '{s2}' is {len}");

    println!("{}", s1.clone());
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
```

**결과 출력**
```
The length of 'Hello' is 5
Hello
```

-----

개선된 코드

```rust
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
```

**결과 출력**
```
The length of 'Hello' is 5
```

출력하는 내용은 기존의 코드와 동일한 결괏값이 출력되지만, 코드는 더 간결한다.