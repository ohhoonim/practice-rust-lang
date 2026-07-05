# 반복문(loop, while, for)을 이용한 흐름 제어

- **요약 목록**
    - **`loop`**: 무한 루프를 생성하며, `break`로 멈추거나 값을 반환할 수 있고 `continue`로 다음 반복으로 건너뜁니다.
    - **`while`**: 조건식이 참(`true`)인 동안 계속 반복 실행합니다.
    - **`for`**: 컬렉션의 요소를 안전하고 효율적으로 순회할 때 가장 많이 사용되는 반복문입니다.

## 1. 무한 반복을 위한 `loop`

`loop` 키워드는 중단하라고 명시적으로 지시하기 전까지 코드 블록을 무한히 반복합니다.

```rust
let mut count = 0;

loop {
    count += 1;
    if count == 3 {
        continue; // 다음 반복으로 건너뜀
    }
    if count == 5 {
        break; // 루프 종료
    }
}
```

### 루프에서 값 반환하기

`loop`의 `break` 뒤에 반환할 값을 적으면, 루프를 종료하면서 해당 값을 표현식처럼 반환할 수 있습니다.

```rust
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2; // 루프를 탈출하며 값을 반환
    }
};
```

## 2. 조건부 반복을 위한 `while`

`while` 문은 루프를 실행하기 전에 조건식을 평가합니다. 조건이 참(`true`)이면 블록을 실행하고, 거짓(`false`)이 되면 루프를 빠져나옵니다.

```rust
let mut number = 3;

while number != 0 {
    number -= 1;
}
```

## 3. 컬렉션 순회를 위한 `for`

배열이나 컬렉션의 요소를 순회할 때는 인덱스를 사용하는 `while`보다 `for` 문을 쓰는 것이 훨씬 안전하고 효율적입니다. 인덱스 범위를 벗어나는 실수(`panic`)를 원천 차단하기 때문입니다.

```rust
let a = [10, 20, 30, 40, 50];

for element in a {
    // element에 배열의 각 값이 차례대로 담김
}
```

### 지정된 횟수만큼 반복하기 (Range)

`for` 문과 표준 라이브러리의 `Range`를 이용하면 특정 횟수만큼 안전하게 반복할 수 있습니다. `rev()`를 붙이면 역순으로 순회합니다.

```rust
// 1부터 3까지 반복 (4는 포함 안 됨)
for number in 1..4 {
    // 1, 2, 3 순서대로 실행
}

// 3부터 1까지 역순으로 반복
for number in (1..4).rev() {
    // 3, 2, 1 순서대로 실행
}
```

현재 단계: 3단계 완료 (다음 단계: 4단계 소유권 Ownership)