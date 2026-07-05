# 문제 2 (FlatMap과 Collect)

## [문제 2] flat_map과 collect를 이용한 중첩 데이터 평탄화

### 설명

자바의 `Stream.flatMap()`처럼 여러 개의 리스트가 중첩된 구조를 하나의 평평한 이터레이터로 펼치고, 이를 최종 수집(`collect`)하는 훈련입니다.

여러 부서(Department)와 각 부서에 속한 직원 이름 목록이 주어질 때, **모든 부서의 직원 이름을 하나의 단일 벡터**로 모으는 함수를 완성하세요.

- 부서 목록을 이터레이터로 바꾼 뒤, `.flat_map()`을 사용하여 내부의 `employees` 벡터들을 1차원으로 펼칩니다.

### 템플릿 코드

```rust
struct Department {
    name: String,
    employees: Vec<String>,
}

fn gather_all_employees(departments: Vec<Department>) -> Vec<String> {
    // 이 부분을 구현하세요.
    // into_iter()와 flat_map()을 활용해 보세요.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flat_map_collect() {
        let depts = vec![
            Department {
                name: "Dev".to_string(),
                employees: vec!["Alice".to_string(), "Bob".to_string()],
            },
            Department {
                name: "Design".to_string(),
                employees: vec!["Charlie".to_string()],
            },
        ];

        let all_employees = gather_all_employees(depts);
        assert_eq!(all_employees, vec!["Alice", "Bob", "Charlie"]);
    }
}
```