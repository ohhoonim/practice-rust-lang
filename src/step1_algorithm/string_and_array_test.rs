use super::*;


#[test]
fn test_reverse() {
    let mut s1 = vec!['h', 'e', 'l', 'l', 'o'];
    reverse_string(&mut s1);
    assert_eq!(s1, vec!['o', 'l', 'l', 'e', 'h']);

    let mut s2 = vec!['H', 'a', 'n', 'n', 'a', 'h'];
    reverse_string(&mut s2);
    assert_eq!(s2, vec!['h', 'a', 'n', 'n', 'a', 'H']);
}


#[test]
fn test_longest_common_prefix() {
    let strs1 = vec!["flower", "flow", "flight"];
    assert_eq!(longest_common_prefix(&strs1), "fl".to_string());

    let strs2 = vec!["dog", "racecar", "car"];
    assert_eq!(longest_common_prefix(&strs2), "".to_string());

    let strs3 = vec!["interspecies", "interstellar", "interstate"];
    assert_eq!(longest_common_prefix(&strs3), "inters".to_string());
}

#[test]
fn test_two_sum_sorted() {
    let nums1 = vec![2, 7, 11, 15];
    assert_eq!(two_sum_sorted(&nums1, 9), vec![0, 1]);

    let nums2 = vec![2, 3, 4];
    assert_eq!(two_sum_sorted(&nums2, 6), vec![0, 2]);

    let nums3 = vec![-1, 0];
    assert_eq!(two_sum_sorted(&nums3, -1), vec![0, 1]);

    let nums4 = vec![-1];
    assert_eq!(two_sum_sorted(&nums4, 10), vec![]);
}
