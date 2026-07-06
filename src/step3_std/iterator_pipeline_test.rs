use std::os::unix::process;

use super::*;

// Filter와 Map 체이닝

#[test]
fn test_filter_map() {
    let nums = vec![1, 2, 3, 4, 5, 6];

    let result = process_numbers(nums);
    assert_eq!(result, vec![4, 16, 36]);
}

// FlatMap과 Collect

#[test]
fn test_flat_map_collect() {
    let depts = vec![
        Department {
            name: "Dev".to_string(),
            employees: vec!["Alice".to_string(), "Bob".to_string()],
        },
        Department {
            name: "Design".to_string(),
            employees: vec!["Charlie".to_string()],
        },
    ];

    let all_employees = gather_all_employees(depts);
    assert_eq!(all_employees, vec!["Alice", "Bob", "Charlie"]);
}

// Reducing

#[test]
fn test_iterator_fold() {
    let products = vec![
        Product {
            name: "Laptop".to_string(),
            price: 1000,
        },
        Product {
            name: "Mouse".to_string(),
            price: 50,
        },
        Product {
            name: "Keyboard".to_string(),
            price: 150,
        },
    ];

    let total = calculate_total_price(&products);
    assert_eq!(total, 1200);
}

#[test]
fn test_string_handling() {
    let first_name = "john";
    let last_name = "doe";

    let fullname = generate_uppercase_fullname(first_name, last_name);
    assert_eq!(fullname, "JOHN DOE");
}

// BufRead

#[test]
fn test_buf_reader() {
    // 메모리 상의 &[u8] 슬라이스는 Read 트레이트를 구현하고 있으므로 가상 파일 역할을 할 수 있습니다.
    let data = b"Hello Rust\nModern Java\nClean Code";

    let result = read_lines_from_stream(&data[..]).unwrap();

    assert_eq!(result.len(), 3);
    assert_eq!(result[0], "Hello Rust");
    assert_eq!(result[1], "Modern Java");
}

// Write 트레이트

#[test]
fn test_write_trait() {
    // Vec<u8> 구조체는 Write 트레이트를 구현하고 있으므로 메모리 버퍼 스트림 역할을 수행합니다.
    let mut buffer = Vec::new();
    let sample_items = vec!["Spring", "Boot", "4"];

    write_items(&mut buffer, &sample_items).unwrap();

    // 버퍼에 쌓인 바이트 데이터를 문자열로 전환하여 검증
    let result_string = String::from_utf8(buffer).unwrap();
    assert_eq!(result_string, "Spring\nBoot\n4\n");
}
