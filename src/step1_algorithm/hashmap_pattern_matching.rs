use std::collections::HashMap;
// 정수 슬라이스 nums와 목표값 target이 주어집니다. 두 수의 합이 target이 되는 두 원소의 인덱스를 벡터에 담아 반환하는 two_sum 함수를 완성하세요. 만족하는 값이 없으면 빈 벡터를 반환합니다.
pub fn two_sum(nums: &[i32], target: i32) -> Vec<usize> {
   let mut map = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&index) = map.get(&complement) {
            return vec![index, i];
        }
        map.insert(num, i);
    }
    vec![]
}

// 주어진 두 문자열 s와 t가 아나그램이면 true, 아니면 false를 반환하는 is_anagram 함수를 완성하세요.
fn is_anagram(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut count = HashMap::new();

    for c in s.chars() {
        *count.entry(c).or_insert(0) += 1;
    }
    
    for c in t.chars() {
        let current = count.entry(c).or_insert(0);
        if *current == 0 {
            return false;
        }
        *current -= 1;
    }

    true
}

// 주어진 문자열 s에서 반복되지 않는 첫 번째 문자를 찾아 그 인덱스를 반환하는 first_uniq_char 함수를 완성하세요. 만약 존재하지 않으면 None을 반환합니다.
fn first_uniq_char(s: &str) -> Option<usize> {
    let mut count = HashMap::new();
    for c in s.chars() {
        *count.entry(c).or_insert(0) += 1;
    }

    for (i, c) in s.chars().enumerate() {
        if let Some(&1) = count.get(&c) {
            return Some(i);
        }
    }

    None
}


#[cfg(test)]
#[path = "hashmap_pattern_matching_test.rs"]
mod hashma_pattern_matching_test; 