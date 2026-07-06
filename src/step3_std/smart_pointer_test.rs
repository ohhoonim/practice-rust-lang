use super::*;

// option chaining

#[test]
fn test_option_chaining() {
    let user1 = User {
        name: "Alice".to_string(),
        score: Some(40),
    };
    let user2 = User {
        name: "Bob".to_string(),
        score: None,
    };

    assert_eq!(get_bonus_score(&user1), 80);
    assert_eq!(get_bonus_score(&user2), 0);
}

// Result 에러 전파

#[test]
fn test_result_propagation() {
    // 성공 케이스: "12" -> 12 -> 제곱해서 144
    assert_eq!(square_number("12"), Ok(144));

    // 실패 케이스: 정수로 바꿀 수 없는 문자열인 경우 에러 반환 검증
    assert!(square_number("abc").is_err());
}

// RefCell

#[test]
fn test_refcell_mutability() {
    let logger = MockLogger::new();

    // 불변 객체인 logger를 사용해 함수를 호출하지만 내부 카운트가 올라가야 함
    logger.send_log("First warning");
    logger.send_log("Second warning");

    assert_eq!(logger.get_count(), 2);
}
