use super::*;

#[test]
fn test_two_sum() {
    let nums1 = vec![2, 7, 11, 15];
    assert_eq!(two_sum(&nums1, 9), vec![0, 1]);

    let nums2 = vec![3, 2, 4];
    assert_eq!(two_sum(&nums2, 6), vec![1, 2]);

    let nums3 = vec![3, 3];
    assert_eq!(two_sum(&nums3, 6), vec![0, 1]);
}

#[test]
fn test_is_anagram() {
    assert_eq!(is_anagram("anagram", "nagaram"), true);
    assert_eq!(is_anagram("rat", "car"), false);
    assert_eq!(is_anagram("a", "ab"), false);
}

#[test]
fn test_first_uniq_char() {
    assert_eq!(first_uniq_char("leetcode"), Some(0)); // 'l'이 첫 번째 고유 문자
    assert_eq!(first_uniq_char("loveleetcode"), Some(2)); // 'v'가 첫 번째 고유 문자
    assert_eq!(first_uniq_char("aabb"), None); // 고유 문자 없음
}
