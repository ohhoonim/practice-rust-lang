# 문제 1 (Filter와 Map 체이닝)

## [문제 1] Iterator의 filter와 map을 활용한 데이터 변환

### 설명

자바 스트림의 가장 대표적인 문법인 `stream().filter().map()` 구조를 Rust의 이터레이터 어댑터로 변환하는 훈련입니다.

정수 벡터를 받아 **짝수만 골라낸 뒤, 그 값들을 제곱**한 새로운 벡터를 반환하는 함수를 완성하세요.

- `.iter()`를 통해 이터레이터를 생성합니다.
- `.filter()`와 `.map()`을 체이닝하여 조건을 적용합니다.
- `.collect()`를 호출하여 최종적으로 `Vec<i32>` 형태로 변환합니다.

### 템플릿 코드

```rust
fn process_numbers(numbers: Vec<i32>) -> Vec<i32> {
    // 이 부분을 구현하세요.
    // 이터레이터 파이프라인을 사용하여 한 줄로 작성해 보세요.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_map() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        // 짝수(2, 4, 6)를 골라내어 제곱 -> [4, 16, 36]
        let result = process_numbers(nums);
        assert_eq!(result, vec![4, 16, 36]);
    }
}
```