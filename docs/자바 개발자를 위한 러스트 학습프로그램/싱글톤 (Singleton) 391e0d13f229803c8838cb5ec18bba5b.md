# 싱글톤 (Singleton)

- **목표:** 프로그램 전체에서 오직 하나의 인스턴스만 존재하고, 어디서나 접근 가능한 싱글톤 패턴을 구현합니다.
- **조건:** Rust는 전역 가변 상태를 안전하게 다루기 위해 엄격한 동시성 제어를 요구합니다. 멀티스레드 환경에서도 안전하게 싱글톤을 초기화하고 접근할 수 있도록 `std::sync::OnceLock`을 활용하여 관용적인 싱글톤 구조를 구현합니다.

### [연습 문제] 싱글톤 패턴 구현

애플리케이션 전역에서 로그 메시지를 카운트하고 관리하는 중앙 `DatabaseConnection` 시스템을 싱글톤으로 구현하세요.

1. `DatabaseConnection` 구조체는 연결 문자열(`connection_string`) 필드를 가집니다.
2. `std::sync::OnceLock`을 전역 변수로 선언하여, 멀티스레드 환경에서도 단 한 번만 안전하게 인스턴스가 생성되도록 제어합니다.
3. `get_instance()` 메서드는 전역 인스턴스의 공유 참조(`&'static DatabaseConnection`)를 반환합니다.

```rust
use std::sync::OnceLock;

#[derive(Debug, PartialEq)]
struct DatabaseConnection {
    connection_string: String,
}

impl DatabaseConnection {
    // 전역 싱글톤 인스턴스를 반환하는 메서드를 구현하세요.
    // OnceLock의 get_or_init 메서드를 활용하여 최초 호출 시에만 인스턴스를 생성하도록 합니다.
    fn get_instance() -> &'static Self {
        // 이 부분을 구현하세요.
        // connection_string의 값은 "mysql://localhost:3306/prod"로 초기화합니다.
    }

    fn query(&self) -> String {
        format!("Executing query on {}", self.connection_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_singleton_instance() {
        // 서로 다른 변수로 인스턴스를 가져옵니다.
        let instance1 = DatabaseConnection::get_instance();
        let instance2 = DatabaseConnection::get_instance();

        // 두 참조가 가리키는 실제 메모리 주소가 동일한지 검증합니다.
        assert!(std::ptr::eq(instance1, instance2));
        assert_eq!(instance1.query(), "Executing query on mysql://localhost:3306/prod");
    }

    #[test]
    fn test_singleton_thread_safety() {
        // 여러 스레드에서 동시에 접근해도 동일한 인스턴스를 가리키는지 검증합니다.
        let handle1 = thread::spawn(|| {
            DatabaseConnection::get_instance()
        });

        let handle2 = thread::spawn(|| {
            DatabaseConnection::get_instance()
        });

        let res1 = handle1.join().unwrap();
        let res2 = handle2.join().unwrap();

        assert!(std::ptr::eq(res1, res2));
    }
}
```

[현재 단계: 생성 패턴 - 싱글톤]