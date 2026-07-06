use super::*;

// Vec 데이터 조작

#[test]
fn test_vec_operations() {
    let mut todo = TodoList::new();

    todo.add_task("Clean room".to_string());
    todo.add_task("Write code".to_string());

    assert_eq!(todo.tasks.len(), 2);

    // 0번 인덱스의 "Clean room"을 제거하며 소유권을 가져옴
    let completed = todo.complete_task(0);
    assert_eq!(completed, "Clean room");

    // 남은 작업 검증
    assert_eq!(todo.tasks[0], "Write code");
    assert_eq!(todo.tasks.len(), 1);
}

// HashMap Entry API

#[test]
fn test_hashmap_entry() {
    let words = vec![
        "apple".to_string(),
        "banana".to_string(),
        "apple".to_string(),
    ];

    let result = count_words(words);
    assert_eq!(result.get("apple"), Some(&2));
    assert_eq!(result.get("banana"), Some(&1));
    assert_eq!(result.get("orange"), None);
}

// HashSet

#[test]
fn test_hashset_operations() {
    let mut manager = UserManager::new();
    // 처음 등록하면 true
    assert_eq!(manager.register_user(1001), true);
    assert_eq!(manager.register_user(1002), true);
    // 중복 등록시 false
    assert_eq!(manager.register_user(1001), false);

    assert_eq!(manager.total_unique_users(), 2);

}


