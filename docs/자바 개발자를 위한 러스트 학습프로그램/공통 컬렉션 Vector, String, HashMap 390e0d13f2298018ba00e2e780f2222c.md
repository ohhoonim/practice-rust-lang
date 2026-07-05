# 공통 컬렉션: Vector, String, HashMap

- **요약 목록**
    - **`Vector` (`Vec<T>`)**: 스택이 아닌 힙 메모리에 **같은 타입의 값들을 연속으로 저장**하며, 크기가 유동적으로 변합니다.
    - **`String`**: 힙 메모리에 UTF-8 형식의 텍스트 데이터를 저장하는 가변 문자열 컬렉션입니다.
    - **`HashMap` (`HashMap<K, V>`)**: 키(Key)와 값(Value)의 쌍을 매핑하여 저장하며, 특정 키를 통한 빠른 데이터 검색에 사용됩니다.

## 1. Vector (`Vec<T>`)

벡터는 단일 데이터 구조에 **동일한 타입의 여러 값**을 이웃하여(메모리상에 연속으로) 저장할 수 있게 해줍니다. 배열과 달리 크기를 마음대로 늘리거나 줄일 수 있습니다.

### 생성 및 추가

```rust
let mut v: Vec<i32> = Vec::new(); // 빈 벡터 생성
let mut v2 = vec![1, 2, 3];       // 초기값이 있는 벡터 생성 매크로

v2.push(4); // 값 추가 (v2는 이제 [1, 2, 3, 4])
```

### 요소 읽기

벡터의 요소에 접근하는 방법은 두 가지가 있으며, 인덱스를 벗어났을 때의 처리 방식이 다릅니다.

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2]; // 1. 인덱스 매칭 (범위 초과 시 프로그램 panic 종료)

match v.get(2) {         // 2. get 메서드 (Option<&T> 반환, 안전함)
    Some(third) => println!("세 번째 요소: {third}"),
    None => println!("해당 요소가 없습니다."),
}
```

## 2. String (문자열 컬렉션)

Rust의 핵심 언어 기능에서는 문자열 슬라이스 `&str`만을 제공하지만, 표준 라이브러리에서는 힙에 할당되고 크기가 늘어날 수 있는 UTF-8 인코딩된 가변 문자열 타입인 `String`을 제공합니다. 이는 사실상 **u8 바이트의 벡터(`Vec<u8>`)를 감싼 구조**입니다.

### 생성 및 수정

```rust
let mut s = String::new();
let data = "초기 내용";
let mut s2 = data.to_string(); // &str을 String으로 변환

s2.push_str(" 추가"); // 문자열 덧붙이기
```

### 문자열 인덱싱 주의사항

Rust의 `String`은 UTF-8로 인코딩되어 있기 때문에, 알파벳은 1바이트를 차지하지만 한글 같은 문자는 3바이트를 차지합니다. 따라서 `s2[0]`과 같은 **직접적인 정수 인덱싱을 지원하지 않습니다.** 대신 슬라이스 구문을 사용하거나 `.chars()` 메서드를 이용해 안전하게 순회해야 합니다.

```rust
for c in "안녕하세요".chars() {
    // 한 글자씩 안전하게 출력
}
```

## 3. HashMap (`HashMap<K, V>`)

해시맵은 `K` 타입의 키(Key)에 `V` 타입의 값(Value)을 매핑하여 저장하는 데이터 구조입니다. 데이터를 인덱스가 아닌, 임의의 타입으로 만든 키를 통해 찾고 싶을 때 사용합니다.

### 생성 및 사용

```rust
use std::collections::HashMap; // 해시맵은 기본 프렐루드에 없어 가져와야 함

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10); // 데이터 삽입
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score: Option<&i32> = scores.get(&team_name); // 값 가져오기 (Option 반환)
```

### 소유권 주의점

`String`처럼 힙에 데이터를 저장하는 타입을 해시맵의 키나 값으로 넘기면, 해당 값들의 소유권이 해시맵 내부로 이동(Move)하게 됩니다.