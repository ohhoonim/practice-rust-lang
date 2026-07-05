# 첫 번째 고유 문자 찾기 (First Unique Character)

- **목표:** 문자열을 선행 순회하며 각 문자의 등장 횟수를 기록한 뒤, 다시 문자열을 처음부터 확인하여 단 한 번만 등장하는 첫 번째 문자의 인덱스를 찾습니다.
- **조건:** 첫 번째 순회에서는 해시맵을 채우고, 두 번째 순회에서는 해시맵의 데이터를 참조하여 조건에 맞는 인덱스를 정확히 식별하는 문맥 제어를 연습합니다.

### [연습 문제] 첫 번째 고유 문자 찾기 (First Unique Character in a String)

주어진 문자열 `s`에서 반복되지 않는 첫 번째 문자를 찾아 그 인덱스를 반환하는 `first_uniq_char` 함수를 완성하세요. 만약 존재하지 않으면 `None`을 반환합니다.

```rust
use std::collections::HashMap;

fn first_uniq_char(s: &str) -> Option<usize> {
    // 이 부분을 구현하세요.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_uniq_char() {
        assert_eq!(first_uniq_char("leetcode"), Some(0)); // 'l'이 첫 번째 고유 문자
        assert_eq!(first_uniq_char("loveleetcode"), Some(2)); // 'v'가 첫 번째 고유 문자
        assert_eq!(first_uniq_char("aabb"), None); // 고유 문자 없음
    }
}
```