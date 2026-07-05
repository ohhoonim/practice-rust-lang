# 구조체(Struct) 정의 및 인스턴스 생성, 메서드(impl) 구현

- **요약 목록**
    - **구조체 정의**: `struct` 키워드를 사용하여 연관된 여러 필드를 하나의 데이터 타입으로 묶습니다.
    - **인스턴스 생성**: 정의된 구조체를 바탕으로 실제 객체(데이터)를 생성하며, 기본적으로 불변이고 `mut`을 붙여 가변으로 만들 수 있습니다.
    - **메서드 구현**: `impl` 블록 내에 함수를 정의하며, 첫 번째 매개변수로 자신의 인스턴스를 가리키는 `&self`를 받습니다.

## 구조체 (Struct) 정의 및 인스턴스 생성

구조체는 연관된 여러 개의 값을 묶어서 하나의 의미 있는 데이터 타입을 정의할 수 있게 해줍니다. 튜플과 유사하지만 각 데이터 항목에 이름(필드명)을 붙여 의미를 더 명확히 할 수 있습니다.

### 1. 구조체 정의하기

```rust
struct User {
    active: bool,
    username: String,
    sign_in_count: u64,
}
```

### 2. 인스턴스 생성하기

정의한 구조체를 바탕으로 실제 데이터를 채워 넣어 인스턴스를 생성합니다. 필드의 순서는 정의할 때와 달라도 상관없습니다.

```rust
let mut user1 = User {
    active: true,
    username: String::from("someuser123"),
    sign_in_count: 1,
};

// 가변(mut) 인스턴스인 경우 점(.) 연산자로 필드 값을 변경할 수 있습니다.
user1.sign_in_count = 2;
```

## 메서드 (Method) 구현: `impl`

메서드는 함수와 유사하지만, 구조체(또는 열거형, 트레이트)의 문맥 내에서 정의되며 첫 번째 매개변수가 항상 `self`라는 점이 다릅니다. `self`는 메서드를 호출하고 있는 구조체의 인스턴스를 나타냅니다.

구조체에 메서드를 붙이려면 `impl` (Implementation) 블록을 사용합니다.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

// Rectangle 구조체에 대한 구현 블록
impl Rectangle {
    // 사각형의 넓이를 구하는 메서드
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

### `&self`의 의미

- `&self`는 실제로는 `self: &Self`를 줄인 표현입니다. (여기서 `Self`는 `impl`을 구현하고 있는 타입을 가리킵니다.)
- 구조체의 데이터를 읽기만 할 때는 `&self`를 사용합니다.
- 메서드 내부에서 구조체의 필드 값을 변경해야 한다면 `&mut self`를 사용해야 합니다.

### 메서드 호출 방식

메서드를 호출할 때는 인스턴스 변수 뒤에 점(`.`) 연산자를 사용합니다.

```rust
let rect1 = Rectangle { width: 30, height: 50 };
let rect_area = rect1.area(); // rect_area 값은 1500
```

## **연관 함수 (Associated Functions)**

`impl` 블록 안에는 첫 번째 매개변수로 `self`를 받지 않는 함수도 정의할 수 있습니다. 이를 **연관 함수**라고 부르며, 구조체의 인스턴스 없이 구조체 이름 자체로 호출합니다. 타 언어의 스태틱(Static) 메서드와 유사하며, 주로 생성자로 활용됩니다.

```rust
impl Rectangle {
    // 정사각형을 만드는 연관 함수 (생성자)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// 호출 시 콜론 두 개(::)를 사용합니다.
let sq = Rectangle::square(3);
```