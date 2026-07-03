// 주어진 문자 배열 s를 제자리에서 뒤집는 reverse_string 함수를 완성하세요.
pub fn reverse_string(s: &mut Vec<char>) {
    s.reverse()
}

// 주어진 문자열 슬라이스 배열 strs에서 가장 긴 공통 접두사를 반환하는 longest_common_prefix 함수를 완성하세요. 공통 접두사가 없다면 빈 문자열 ""을 반환합니다.
pub fn longest_common_prefix(strs: &[&str]) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let first = strs[0];

    for (i, c) in first.char_indices() {
        for string in &strs[1..] {
            if let Some(other_c) = string.chars().nth(i) {
                if other_c != c {
                    return first[..i].to_string();
                }
            } else {
                return first[..i].to_string();
            }
        }
    }

    first.to_string()
}

// 오름차순으로 정렬된 정수 슬라이스 `numbers`와 목표값 `target`이 주어집니다. 두 수의 합이 `target`이 되는 두 원소의 인덱스(0-indexed)를 벡터에 담아 반환하는 `two_sum_sorted` 함수를 완성하세요.
// 정답은 오직 하나만 존재하며, 같은 원소를 두 번 사용할 수 없습니다. 만족하는 값이 없으면 빈 벡터를 반환합니다.
fn two_sum_sorted(numbers: &[i32], target: i32) -> Vec<usize> {
    if numbers.is_empty() {
        return Vec::new();
    }

    let mut left = 0;
    let mut right = numbers.len() - 1;

    while left < right {
        let current_sum = numbers[left] + numbers[right];

        if current_sum == target {
            return vec![left, right];
        } else if current_sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }

    Vec::new()
}

#[cfg(test)]
#[path = "string_and_array_test.rs"]
mod string_and_array_test;
