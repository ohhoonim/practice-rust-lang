# 문자열 뒤집기 (Reverse String)

- **목표:** 가변 참조(`&mut`)를 사용하여 벡터(`Vec<char>`) 내부의 원소를 제자리(In-place)에서 직접 스왑하는 함수를 구현합니다.
- **조건:** 새로운 배열이나 벡터를 생성하지 않고, 투 포인터(Two-pointer) 방식을 사용하여 주어진 입력 데이터를 직접 수정해야 합니다.

### [연습 문제] 문자열 뒤집기 (Reverse String)

주어진 문자 배열 `s`를 제자리에서 뒤집는 `reverse_string` 함수를 완성하세요.

```rust
fn reverse_string(s: &mut Vec<char>) {
    // 이 부분을 구현하세요.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        let mut s1 = vec!['h', 'e', 'l','l', 'o'];
        reverse_string(&mut s1);
        assert_eq!(s1, vec!['o', 'l', 'l', 'e', 'h']);

        let mut s2 = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        reverse_string(&mut s2);
        assert_eq!(s2, vec!['h', 'a', 'n', 'n', 'a', 'H']);
    }
}
```