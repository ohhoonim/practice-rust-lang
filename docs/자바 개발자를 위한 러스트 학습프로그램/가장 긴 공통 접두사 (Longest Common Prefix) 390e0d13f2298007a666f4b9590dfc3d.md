# 가장 긴 공통 접두사 (Longest Common Prefix)

- **목표:** 문자열 배열(또는 슬라이스)을 받아 모든 문자열이 공통으로 가지고 있는 가장 긴 앞부분 문자열(접두사)을 찾는 함수를 구현합니다.
- **조건:** Rust의 문자열 슬라이스(`&str`) 특성과 인덱스 범위, 그리고 반복문 제어 메커니즘(`break`, `return` 등)을 올바르게 활용해야 합니다.

### [연습 문제] 가장 긴 공통 접두사 (Longest Common Prefix)

주어진 문자열 슬라이스 배열 `strs`에서 가장 긴 공통 접두사를 반환하는 `longest_common_prefix` 함수를 완성하세요. 공통 접두사가 없다면 빈 문자열 `""`을 반환합니다.

```rust
fn longest_common_prefix(strs: &[&str]) -> String {
    // 이 부분을 구현하세요.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        let strs1 = vec!["flower", "flow", "flight"];
        assert_eq!(longest_common_prefix(&strs1), "fl".to_string());

        let strs2 = vec!["dog", "racecar", "car"];
        assert_eq!(longest_common_prefix(&strs2), "".to_string());

        let strs3 = vec!["interspecies", "interstellar", "interstate"];
        assert_eq!(longest_common_prefix(&strs3), "inters".to_string());
    }
}
```