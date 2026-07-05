use super::*;

// Chain of Responsibility

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
    assert_eq!(
        client_handler.handle(&valid_request),
        Ok("Success".to_string())
    );
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

// Command

#[test]
fn test_command_pattern_execute_and_undo() {
    let mut editor = Editor::new();
    let mut cmd = IncrementCommand::new(5);

    // 1. 커맨드 실행 (0 + 5 = 5)
    cmd.execute(&mut editor);
    assert_eq!(editor.value, 5);

    // 2. 커맨드 취소 (5 - 5 = 0)
    cmd.undo(&mut editor);
    assert_eq!(editor.value, 0);
}

// Interpreter

#[test]
fn test_interpreter_pattern() {
    // 수식 표현: (5 + 10) - 3
    let expression: Box<dyn Expression> = Box::new(Sub::new(
        Box::new(Add::new(
            Box::new(Number::new(5)),
            Box::new(Number::new(10)),
        )),
        Box::new(Number::new(3)),
    ));

    // 인터프리터를 통한 수식 평가 수행
    assert_eq!(expression.interpret(), 12);
}

// Iterator

#[test]
fn test_iterator_pattern() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let mut iterator = EvenIterator::new(numbers);

    // 반복자를 통해 짝수만 순차적으로 꺼내지는지 검증
    assert_eq!(iterator.next(), Some(2));
    assert_eq!(iterator.next(), Some(4));
    assert_eq!(iterator.next(), Some(6));
    assert_eq!(iterator.next(), Some(8));
    assert_eq!(iterator.next(), None);
}

// Mediator

#[test]
fn test_mediator_pattern() {
    let mut chat_room = ChatRoom::new();
    chat_room.register("Alice");
    chat_room.register("Bob");
    chat_room.register("Charlie");

    let alice = User::new("Alice", &chat_room);

    // Alice가 메시지를 보내면 중재자(ChatRoom)가 Bob과 Charlie에게 중계해야 함
    let logs = alice.send_message("Hello, everyone!");

    assert_eq!(logs.len(), 2);
    assert_eq!(logs[0], "Bob received from Alice: Hello, everyone!");
    assert_eq!(logs[1], "Charlie received from Alice: Hello, everyone!");
}

// Memento

#[test]
fn test_memento_pattern() {
    let mut editor = EditorM::new();

    // 1. 첫 번째 상태 작성 후 저장
    editor.type_text("Hello, ");
    let saved_state1 = editor.save();

    // 2. 두 번째 상태 작성 후 저장
    editor.type_text("World!");
    let saved_state2 = editor.save();

    // 3. 추가 작성
    editor.type_text(" Extra text.");
    assert_eq!(editor.contents, "Hello, World! Extra text.");

    // 4. 두 번째 저장 시점으로 복구 (Hello, World!)
    editor.restore(&saved_state2);
    assert_eq!(editor.contents, "Hello, World!");

    // 5. 첫 번째 저장 시점으로 복구 (Hello, )
    editor.restore(&saved_state1);
    assert_eq!(editor.contents, "Hello, ");
}

// Observer

#[test]
fn test_observer_pattern() {
    let mut station = WeatherStation::new();

    // 관찰자 인스턴스를 공유 및 가변 참조가 가능하도록 래핑하여 생성
    let display1 = Rc::new(RefCell::new(CurrentConditionsDisplay::new()));
    let display2 = Rc::new(RefCell::new(CurrentConditionsDisplay::new()));

    station.register_observer(Rc::clone(&display1) as Rc<RefCell<dyn Observer>>);
    station.register_observer(Rc::clone(&display2) as Rc<RefCell<dyn Observer>>);

    // 상태 변경 및 자동 통지 수행
    station.set_measurements(25.5);

    // 모든 관찰자들의 상태가 자동으로 업데이트되었는지 검증
    assert_eq!(display1.borrow().temperature, 25.5);
    assert_eq!(display2.borrow().temperature, 25.5);

    // 다시 한 번 상태 변경
    station.set_measurements(30.0);
    assert_eq!(display1.borrow().temperature, 30.0);
}

// State

#[test]
fn test_state_pattern() {
    let mut fan = Fan::new();

    // 초기 상태: Off -> 버튼 클릭 -> Low 상태로 전환
    assert_eq!(fan.press_button(), "Turning fan on to low speed.");

    // 현재 상태: Low -> 버튼 클릭 -> Off 상태로 전환
    assert_eq!(fan.press_button(), "Turning fan off.");

    // 다시 Off -> 버튼 클릭 -> Low 상태로 전환
    assert_eq!(fan.press_button(), "Turning fan on to low speed.");
}

// Strategy

#[test]
fn test_strategy_pattern() {
    // 1. 기본 전략(Normal)으로 카트 생성
    let mut cart = ShoppingCart::new(Box::new(NormalStrategy));
    assert_eq!(cart.checkout(10000), 10000);

    // 2. 런타임에 할인 전략(Discount)으로 교체
    cart.set_strategy(Box::new(DiscountStrategy));
    assert_eq!(cart.checkout(10000), 9000);
}

// Template Method

#[test]
fn test_template_method_pdf() {
    let pdf_miner = PdfMiner;
    let expected_log = "Opening PDF file.\nExtracting data...\nClosing PDF file.";

    // 템플릿 메서드 실행 결과 검증
    assert_eq!(pdf_miner.mine(), expected_log);
}

#[test]
fn test_template_method_csv() {
    let csv_miner = CsvMiner;
    let expected_log = "Opening CSV file.\nExtracting data...\nClosing CSV file.";

    // 템플릿 메서드 실행 결과 검증
    assert_eq!(csv_miner.mine(), expected_log);
}

// Visitor

#[test]
fn test_visitor_pattern() {
    // 도형 구조들의 리스트 구성 (구조 변경 없음)
    let shapes: Vec<Box<dyn ShapeElement>> = vec![
        Box::new(Circle::new(10.0)),        // 면적: 10 * 10 * 3.14 = 314.0
        Box::new(Rectangle::new(5.0, 4.0)), // 면적: 5 * 4 = 20.0
    ];

    let area_visitor = AreaVisitor::new();

    // 각 도형 구조를 순회하며 방문자 수용
    for shape in shapes.iter() {
        shape.accept(&area_visitor);
    }

    // 총 면적 합계 검증 (314.0 + 20.0 = 334.0)
    assert_eq!(area_visitor.get_total_area(), 334.0);
}
