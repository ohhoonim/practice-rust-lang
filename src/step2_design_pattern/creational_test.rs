use super::*;

// factory method

#[test]
fn test_factory_method() {
    let win_dialog = WindowsDialog;
    assert_eq!(
        win_dialog.render_window(),
        "Dialog rendering with: [Windows Button]"
    );

    let html_dialog = HtmlDialog;
    assert_eq!(
        html_dialog.render_window(),
        "Dialog rendering with: <button>HTML Button</button>"
    );
}

// 팩토리를 소비하는 클라이언트 코드 예시
fn client_code(factory: &dyn GuiFactory) -> (String, String) {
    let button = factory.create_button();
    let checkbox = factory.create_checkbox();
    (button.paint(), checkbox.paint())
}

// Abstract Factory

#[test]
fn test_abstract_factory() {
    let mac_factory = MacFactory;
    let win_factory = WinFactory;

    assert_eq!(
        client_code(&mac_factory),
        ("Mac Button".to_string(), "Mac Checkbox".to_string())
    );
    assert_eq!(
        client_code(&win_factory),
        ("Win Button".to_string(), "Win Checkbox".to_string())
    );
}

// Builder

#[test]
fn test_builder_pattern() {
    // 모든 값을 수동으로 설정한 경우
    let config1 = ServerConfigBuilder::new()
        .host("0.0.0.0")
        .port(443)
        .timeout(60)
        .max_connections(5000)
        .build();

    assert_eq!(
        config1,
        ServerConfig {
            host: "0.0.0.0".to_string(),
            port: 443,
            timeout: 60,
            max_connections: 5000,
        }
    );

    // 설정을 생략하여 기본값이 적용되는 경우
    let config2 = ServerConfigBuilder::new().host("192.168.1.1").build();

    assert_eq!(
        config2,
        ServerConfig {
            host: "192.168.1.1".to_string(),
            port: 8080,
            timeout: 30,
            max_connections: 1000,
        }
    );
}

// Prototype

#[test]
fn test_prototype_clone() {
    let original_cell = Cell::new("Alpha", vec![1, 0, 1, 1, 0]);

    // 프로토타입 복제 실행
    let cloned_cell = original_cell.clone();

    // 원본 세포는 원래 상태를 유지해야 함
    assert_eq!(original_cell.name, "Alpha".to_string());

    // 복제된 세포는 이름 뒤에 _clone이 붙고 DNA는 동일해야 함
    assert_eq!(
        cloned_cell,
        Cell {
            name: "Alpha_clone".to_string(),
            dna_sequence: vec![1, 0, 1, 1, 0],
        }
    );
}

// Singleton

use std::thread;

#[test]
fn test_singleton_instance() {
    // 서로 다른 변수로 인스턴스를 가져옵니다.
    let instance1 = DatabaseConnection::get_instance();
    let instance2 = DatabaseConnection::get_instance();

    // 두 참조가 가리키는 실제 메모리 주소가 동일한지 검증합니다.
    assert!(std::ptr::eq(instance1, instance2));
    assert_eq!(
        instance1.query(),
        "Executing query on mysql://localhost:3306/prod"
    );
}

#[test]
fn test_singleton_thread_safety() {
    // 여러 스레드에서 동시에 접근해도 동일한 인스턴스를 가리키는지 검증합니다.
    let handle1 = thread::spawn(|| DatabaseConnection::get_instance());

    let handle2 = thread::spawn(|| DatabaseConnection::get_instance());

    let res1 = handle1.join().unwrap();
    let res2 = handle2.join().unwrap();

    assert!(std::ptr::eq(res1, res2));
}
