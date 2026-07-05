# 프로토타입 (Prototype)

- **목표:** 기존 인스턴스를 복제(Clone)하여 새로운 인스턴스를 생성하는 프로토타입 패턴을 구현합니다.
- **조건:** Rust에서는 표준 라이브러리의 `Clone` 트레이트가 프로토타입 패턴의 역할을 완벽하게 대체합니다. `#[derive(Clone)]`을 사용하는 방법과, 깊은 복사(Deep Copy)나 커스텀 복제 로직이 필요할 때 `Clone` 트레이트를 수동으로 직접 구현(`impl Clone`)하는 방법을 연습합니다.

### [연습 문제] 프로토타입 패턴 구현

게임 내 세포 분열 시스템을 프로토타입 패턴으로 구현하세요.

1. `Cell` 구조체는 세포의 이름(`name`)과 내부 유전자 데이터(`dna_sequence`)를 가집니다.
2. `Clone` 트레이트를 수동으로 구현하여, 세포가 복제(`clone`)될 때 이름 뒤에 `"_clone"`이 자동으로 붙고 유전자 데이터는 그대로 복사되도록 만드세요.

```rust
#[derive(Debug, PartialEq)]
struct Cell {
    name: String,
    dna_sequence: Vec<u8>,
}

impl Cell {
    fn new(name: &str, dna: Vec<u8>) -> Self {
        Cell {
            name: name.to_string(),
            dna_sequence: dna,
        }
    }
}

// Clone 트레이트를 수동으로 구현하여 프로토타입 복제 메커니즘을 정의합니다.
impl Clone for Cell {
    // 이 부분을 구현하세요.
    // 복제된 세포의 name은 원래 name 뒤에 "_clone"이 추가되어야 합니다. (예: "Alpha" -> "Alpha_clone")
    // dna_sequence는 그대로 복사합니다.
    fn clone(&self) -> Self {
        // 이 부분을 구현하세요.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prototype_clone() {
        let original_cell = Cell::new("Alpha", vec![1, 0, 1, 1, 0]);

        // 프로토타입 복제 실행
        let cloned_cell = original_cell.clone();

        // 원본 세포는 원래 상태를 유지해야 함
        assert_eq!(original_cell.name, "Alpha".to_string());

        // 복제된 세포는 이름 뒤에 _clone이 붙고 DNA는 동일해야 함
        assert_eq!(cloned_cell, Cell {
            name: "Alpha_clone".to_string(),
            dna_sequence: vec![1, 0, 1, 1, 0],
        });
    }
}
```

[현재 단계: 생성 패턴 - 프로토타입]