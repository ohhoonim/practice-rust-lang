# 인터프리터 (Interpreter)

- **목표:** 추상 구문 트리(AST)의 노드를 표현하는 클래스/구조체들을 정의하고, 각 노드가 문맥(Context)을 해석하여 결과를 반환하는 인터프리터 패턴을 구현합니다.
- **조건:** Rust에서는 인터프리터 패턴의 트리 구조를 표현할 때 `enum`을 활용하면 매우 안전하고 직관적인 도메인 특화 언어(DSL) 평가기를 매칭(Pattern Matching)으로 구현할 수 있습니다.

### [연습 문제] 인터프리터 패턴 구현

간단한 사칙연산 수식(더하기, 빼기)을 해석하고 평가(Evaluate)하는 인터프리터 시스템을 구현하세요.

1. **표현식(Expression) 인터페이스:** `Expression` 트레이트는 값을 계산하는 `interpret` 메서드를 가집니다.
2. **종단 표현식(Terminal Expression):** `Number` 구조체는 실제 숫자 값을 가지며, 값을 그대로 반환합니다.
3. **비종단 표현식(Non-terminal Expression):** `Add` 구조체는 두 개의 하위 표현식을 소유하며, 두 식의 결과를 더합니다. `Sub` 구조체는 앞의 식 결과에서 뒤의 식 결과를 뺍니다.

```rust
// 1. 공통 추상 표현식 인터페이스 (Abstract Expression)
trait Expression {
    fn interpret(&self) -> i32;
}

// 2. 종단 표현식 (Terminal Expression) - 숫자
struct Number {
    value: i32,
}

impl Number {
    fn new(value: i32) -> Self {
        Number { value }
    }
}

impl Expression for Number {
    fn interpret(&self) -> i32 {
        self.value
    }
}

// 3. 비종단 표현식 (Non-terminal Expression) - 덧셈
struct Add {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Add {
    fn new(left: Box<dyn Expression>, right: Box<dyn Expression>) -> Self {
        Add { left, right }
    }
}

// Add에 Expression 트레이트를 구현하세요.
// interpret 메서드는 left의 해석 결과와 right의 해석 결과를 더해 반환합니다.
impl Expression for Add {
    // 이 부분을 구현하세요.
}

// 4. 비종단 표현식 (Non-terminal Expression) - 뺄셈
struct Sub {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Sub {
    fn new(left: Box<dyn Expression>, right: Box<dyn Expression>) -> Self {
        Sub { left, right }
    }
}

// Sub에 Expression 트레이트를 구현하세요.
// interpret 메서드는 left의 해석 결과에서 right의 해석 결과를 빼서 반환합니다.
impl Expression for Sub {
    // 이 부분을 구현하세요.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpreter_pattern() {
        // 수식 표현: (5 + 10) - 3
        let expression: Box<dyn Expression> = Box::new(Sub::new(
            Box::new(Add::new(
                Box::new(Number::new(5)),
                Box::new(Number::new(10)),
            )),
            Box::new(Number::new(3)),
        ));

        // 인터프리터를 통한 수식 평가 수행
        assert_eq!(expression.interpret(), 12);
    }
}
```

[현재 단계: 행동 패턴 - 인터프리터]