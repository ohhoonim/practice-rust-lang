# 팩토리 메서드 (Factory Method)

- **목표:** Rust의 `Trait`와 연관 타입(Associated Types) 혹은 다형성을 활용하여 구체적인 타입을 직접 명시하지 않고 객체를 생성하는 팩토리 메서드 패턴을 구현합니다.
- **조건:** Rust에는 클래스와 상속이 없으므로, 인터페이스 역할은 `trait`가 수행하며 구체적인 생성 로직은 해당 트레이트를 구현하는 개별 구조체(`struct`)가 담당하도록 설계합니다.

### [연습 문제] 팩토리 메서드 패턴 구현

다양한 종류의 버튼(Button)을 생성하는 다이얼로그(Dialog) 시스템을 팩토리 메서드 패턴으로 구현하세요.

1. `Button` 트레이트는 `render` 메서드를 가집니다.
2. `Dialog` 트레이트는 `create_button`이라는 **팩토리 메서드**를 가집니다. 이 메서드는 `Button` 트레이트를 구현한 객체를 반환해야 합니다. (여기서는 동적 디스패치 `Box<dyn Button>`를 사용합니다.)
3. `WindowsDialog`와 `HtmlDialog` 구조체는 각각 `Dialog`를 구현하며, 자신에게 맞는 버튼(`WindowsButton`, `HtmlButton`)을 생성하여 반환합니다.

```rust
// 1. 제품(Product) 인터페이스 및 구체적인 제품들
trait Button {
    fn render(&self) -> String;
}

struct WindowsButton;
impl Button for WindowsButton {
    fn render(&self) -> String {
        "[Windows Button]".to_string()
    }
}

struct HtmlButton;
impl Button for HtmlButton {
    fn render(&self) -> String {
        "<button>HTML Button</button>".to_string()
    }
}

// 2. 창조자(Creator) 인터페이스
trait Dialog {
    // 팩토리 메서드: 하위 구조체에서 구체적인 객체 생성을 결정합니다.
    fn create_button(&self) -> Box<dyn Button>;

    // 팩토리 메서드를 활용한 공통 비즈니스 로직
    fn render_window(&self) -> String {
        let button = self.create_button();
        format!("Dialog rendering with: {}", button.render())
    }
}

// 3. 구체적인 창조자(Concrete Creator)들
struct WindowsDialog;
impl Dialog for WindowsDialog {
    // 이 부분을 구현하세요.
}

struct HtmlDialog;
impl Dialog for HtmlDialog {
    // 이 부분을 구현하세요.
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
```