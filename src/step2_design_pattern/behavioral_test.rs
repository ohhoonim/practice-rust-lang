use super::*;

// Adapter

#[test]
fn test_adapter_pattern() {
    let round_peg = LegacyRoundPeg::new(5.0);
    let adapter = SquarePegAdapter::new(round_peg);

    // 반지름이 5.0인 원의 외접 사각형 한 변의 길이는 10.0이 되므로 면적은 100.0입니다.
    assert_eq!(adapter.get_area(), 100.0);
}

// Bridge

#[test]
fn test_bridge_pattern_tv() {
    let tv = Box::new(Tv { on: false });
    let mut remote = Remote::new(tv);

    assert_eq!(remote.is_device_on(), false);
    remote.toggle_power();
    assert_eq!(remote.is_device_on(), true);
    remote.toggle_power();
    assert_eq!(remote.is_device_on(), false);
}

#[test]
fn test_bridge_pattern_radio() {
    let radio = Box::new(Radio { on: true });
    let mut remote = Remote::new(radio);

    assert_eq!(remote.is_device_on(), true);
    remote.toggle_power();
    assert_eq!(remote.is_device_on(), false);
}

// Composite

#[test]
fn test_composite_pattern() {
    let mut root = Directory::new("root");
    let file1 = Box::new(File::new("file1.txt", 100));
    let file2 = Box::new(File::new("file2.txt", 200));

    root.add(file1);
    root.add(file2);

    let mut sub_dir = Directory::new("sub");
    let file3 = Box::new(File::new("file3.txt", 50));
    sub_dir.add(file3);

    root.add(Box::new(sub_dir));

    // 전체 크기 계산: file1(100) + file2(200) + sub_dir(file3(50)) = 350
    assert_eq!(root.get_size(), 350);
}

// Decorator

#[test]
fn test_decorator_pattern() {
    // 1. 기본 커피 (2000)
    let coffee: Box<dyn Coffee> = Box::new(SimpleCoffee);
    assert_eq!(coffee.get_cost(), 2000);

    // 2. 우유 추가 (2000 + 500 = 2500)
    let milk_coffee: Box<dyn Coffee> = Box::new(MilkDecorator::new(coffee));
    assert_eq!(milk_coffee.get_cost(), 2500);

    // 3. 설탕 추가 (2500 + 200 = 2700)
    let sugar_milk_coffee: Box<dyn Coffee> = Box::new(SugarDecorator::new(milk_coffee));
    assert_eq!(sugar_milk_coffee.get_cost(), 2700);
}

// Facade
#[test]
fn test_facade_pattern() {
    let computer = ComputerFacade::new();
    let boot_log = computer.start_computer();

    let expected_log = concat!(
        "CPU frozen.\n",
        "Memory loaded bootloader.\n",
        "HardDrive read boot sector.\n",
        "CPU jumped to boot address.\n",
        "CPU executing commands."
    );

    assert_eq!(boot_log, expected_log);
}

// Flyweight

#[test]
fn test_flyweight_pattern() {
    let mut factory = TreeFactory::new();

    // 동일한 종류의 나무 타입을 팩토리로부터 획득
    let type_oak1 = factory.get_tree_type("Oak");
    let type_oak2 = factory.get_tree_type("Oak");
    let type_pine = factory.get_tree_type("Pine");

    // 두 Oak 타입은 물리적으로 동일한 메모리 주소를 가리키고 있는지 검증 (공유 확인)
    assert!(Rc::ptr_eq(&type_oak1, &type_oak2));
    assert!(!Rc::ptr_eq(&type_oak1, &type_pine));

    // 외래 상태(좌표)를 부여하여 개별 객체 생성
    let forest = vec![
        Tree::new(10, 20, type_oak1),
        Tree::new(50, 60, type_oak2),
        Tree::new(100, 200, type_pine),
    ];

    assert_eq!(forest[0].tree_type.name, "Oak");
    assert_eq!(forest[1].tree_type.name, "Oak");
    assert_eq!(forest[2].tree_type.name, "Pine");

    // 팩토리 내부에 캐싱된 TreeType은 총 2개여야 함
    assert_eq!(factory.tree_types.len(), 2);
}

// Proxy

#[test]
fn test_proxy_pattern_allowed() {
    let internet = ProxyInternet::new();

    // 허용된 사이트 접근
    assert_eq!(
        internet.connect_to("rust-lang.org"),
        "Connecting to rust-lang.org"
    );
}

#[test]
fn test_proxy_pattern_denied() {
    let internet = ProxyInternet::new();

    // 차단된 사이트 접근
    assert_eq!(internet.connect_to("banned-site.com"), "Access Denied");
    assert_eq!(internet.connect_to("malware.io"), "Access Denied");
}
