# 책임 연쇄 (Chain of Responsibility)

- **목표:** 하나의 요청을 처리할 수 있는 여러 처리기(Handler)들을 사슬(Chain)처럼 연결하여, 각 처리기가 요청을 직접 처리하거나 다음 처리기로 전달하는 책임 연쇄 패턴을 구현합니다.
- **조건:** Rust에서는 소유권과 수명 주기 제약 때문에 단순한 링크드 리스트 형태의 구조를 포인터로 구현하기 까다로울 수 있습니다. 여기서는 가독성과 직관성을 위해 개별 처리기 구조체가 다음 처리기(`Option<Box<dyn Handler>>`)를 소유하는 형태로 사슬을 구성하는 패턴을 연습합니다.

### [연습 문제] 책임 연쇄 패턴 구현

로그인 요청의 보안 등급을 단계별로 검증하는 인증 시스템(Authentication Pipeline)을 책임 연쇄 패턴으로 구현하세요.

1. **인터페이스:** `Handler` 트레이트는 다음 처리기를 설정하는 `set_next`와 요청을 처리하는 `handle` 메서드를 가집니다.
2. **구체적인 처리기:** * `ClientCheckHandler`: 요청이 비어있지 않은지 검증합니다. ("Client OK" 또는 에러 반환)
    - `RoleCheckHandler`: 사용자의 권한이 적절한지 검증합니다. ("Role OK" 또는 에러 반환)
3. 각 처리기는 자신의 검증을 통과하면 사슬에 연결된 다음 처리기(`next`)의 `handle`을 호출하고, 다음 처리기가 없다면 최종 성공 메시지(`"Success"`)를 반환합니다. 검증에 실패하면 즉시 해당 에러를 반환하여 사슬을 끊습니다.

```rust
// 요청 데이터 구조체
struct Request {
    username: String,
    role: String,
}

// 1. 공통 처리기 인터페이스 (Handler)
trait Handler {
    fn set_next(&mut self, next: Box<dyn Handler>);
    fn handle(&self, request: &Request) -> Result<String, String>;
}

// 2. 구체적인 처리기 1: 클라이언트 기본 검증
struct ClientCheckHandler {
    next: Option<Box<dyn Handler>>,
}

impl ClientCheckHandler {
    fn new() -> Self {
        ClientCheckHandler { next: None }
    }
}

impl Handler for ClientCheckHandler {
    fn set_next(&mut self, next: Box<dyn Handler>) {
        self.next = Some(next);
    }

    // handle 메서드를 구현하세요.
    // request.username이 빈 문자열("")이면 Err("Invalid Username".to_string())을 즉시 반환합니다.
    // 검증을 통과했을 때, self.next가 Some(handler)이면 다음 핸들러의 handle을 호출하고, None이면 Ok("Success".to_string())를 반환합니다.
    fn handle(&self, request: &Request) -> Result<String, String> {
        // 이 부분을 구현하세요.
    }
}

// 3. 구체적인 처리기 2: 권한 검증
struct RoleCheckHandler {
    next: Option<Box<dyn Handler>>,
}

impl RoleCheckHandler {
    fn new() -> Self {
        RoleCheckHandler { next: None }
    }
}

impl Handler for RoleCheckHandler {
    fn set_next(&mut self, next: Box<dyn Handler>) {
        self.next = Some(next);
    }

    // handle 메서드를 구현하세요.
    // request.role이 "admin"이 아니면 Err("Access Denied for Role".to_string())을 즉시 반환합니다.
    // 검증을 통과했을 때, self.next가 Some(handler)이면 다음 핸들러의 handle을 호출하고, None이면 Ok("Success".to_string())를 반환합니다.
    fn handle(&self, request: &Request) -> Result<String, String> {
        // 이 부분을 구현하세요.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_of_responsibility_success() {
        let mut client_handler = ClientCheckHandler::new();
        let role_handler = Box::new(RoleCheckHandler::new());

        // 체인 연결: ClientCheckHandler -> RoleCheckHandler
        client_handler.set_next(role_handler);

        let valid_request = Request {
            username: "admin_user".to_string(),
            role: "admin".to_string(),
        };

        // 모든 체인을 통과하여 최종 성공해야 함
        assert_eq!(client_handler.handle(&valid_request), Ok("Success".to_string()));
    }

    #[test]
    fn test_chain_of_responsibility_fail_at_first() {
        let mut client_handler = ClientCheckHandler::new();
        let role_handler = Box::new(RoleCheckHandler::new());
        client_handler.set_next(role_handler);

        let invalid_user_request = Request {
            username: "".to_string(),
            role: "admin".to_string(),
        };

        // 첫 번째 링크(ClientCheck)에서 실패하여 사슬이 끊겨야 함
        assert_eq!(
            client_handler.handle(&invalid_user_request),
            Err("Invalid Username".to_string())
        );
    }

    #[test]
    fn test_chain_of_responsibility_fail_at_second() {
        let mut client_handler = ClientCheckHandler::new();
        let role_handler = Box::new(RoleCheckHandler::new());
        client_handler.set_next(role_handler);

        let invalid_role_request = Request {
            username: "guest_user".to_string(),
            role: "guest".to_string(),
        };

        // 첫 번째는 통과하지만 두 번째 링크(RoleCheck)에서 실패해야 함
        assert_eq!(
            client_handler.handle(&invalid_role_request),
            Err("Access Denied for Role".to_string())
        );
    }
}
```

[현재 단계: 행동 패턴 - 책임 연쇄]