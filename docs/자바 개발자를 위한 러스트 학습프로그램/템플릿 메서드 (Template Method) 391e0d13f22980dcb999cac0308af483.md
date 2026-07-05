# 템플릿 메서드 (Template Method)

- **목표:** 알고리즘의 뼈대를 상위 클래스(또는 기본 구현)에 정의하고, 구체적인 처리 단계는 서브클래스(또는 개별 구현체)에서 재정의하여 알고리즘의 구조를 변경하지 않고 특정 단계들을 재정의하는 템플릿 메서드 패턴을 구현합니다.
- **조건:** Rust는 전통적인 클래스 상속을 지원하지 않으므로, 기본 동작(템플릿 메서드)을 트레이트의 기본 메서드(Default Method)로 정의하고, 커스텀이 필요한 단계들은 기본 구현이 없는 메서드로 선언하여 구현체가 이를 강제하도록 설계합니다.

### [연습 문제] 템플릿 메서드 패턴 구현

자원을 빌려서 처리한 후 반드시 반환해야 하는 데이터 마이닝 알고리즘의 흐름을 템플릿 메서드 패턴으로 구현하세요.

1. **인터페이스:** `DataMiner` 트레이트는 알고리즘의 작업 순서를 제어하는 `mine` 메서드(템플릿 메서드)를 기본 구현으로 가집니다.
2. **가변 단계:** `open_file`과 `close_file` 메서드는 각 파일 형식에 맞게 구현체가 직접 정의해야 합니다.
3. `mine` 메서드는 `open_file` 호출 -> "Extracting data..." 로그 추가 -> `close_file` 호출 순서로 알고리즘 골격을 수행하고 전체 로그를 반환합니다.

```rust
// 1. 알고리즘의 골격과 단계를 정의하는 트레이트 (Abstract Class 역할)
trait DataMiner {
    // 구현체가 반드시 정의해야 하는 구체적인 단계들
    fn open_file(&self) -> String;
    fn close_file(&self) -> String;

    // 알고리즘의 골격을 정의하는 템플릿 메서드
    // 순서대로 실행하여 각 단계의 로그를 줄바꿈('\n')으로 연결한 하나의 String을 반환하세요.
    // 실행 순서:
    // 1. self.open_file()
    // 2. "Extracting data...".to_string()
    // 3. self.close_file()
    fn mine(&self) -> String {
        // 이 부분을 구현하세요.
    }
}

// 2. 구체적인 구현체 1 - PDF 마이너 (Concrete Class)
struct PdfMiner;
impl DataMiner for PdfMiner {
    fn open_file(&self) -> String {
        "Opening PDF file.".to_string()
    }
    fn close_file(&self) -> String {
        "Closing PDF file.".to_string()
    }
}

// 3. 구체적인 구현체 2 - CSV 마이너 (Concrete Class)
struct CsvMiner;
impl DataMiner for CsvMiner {
    fn open_file(&self) -> String {
        "Opening CSV file.".to_string()
    }
    fn close_file(&self) -> String {
        "Closing CSV file.".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
```

[현재 단계: 행동 패턴 - 템플릿 메서드]