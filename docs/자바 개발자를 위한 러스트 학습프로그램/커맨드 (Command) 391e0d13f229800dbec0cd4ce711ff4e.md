# 커맨드 (Command)

- **목표:** 실행할 작업이나 요청을 하나의 독립된 객체로 캡슐화하여, 메서드 호출을 매개변수화하고 실행 취소(Undo) 메커니즘을 지원하는 커맨드 패턴을 구현합니다.
- **조건:** Rust에서는 가변 상태를 변경해야 하므로 커맨드 인터페이스(`trait`)의 실행 메서드에 `&mut self` 또는 상태를 가지는 리시버(Receiver) 객체를 적절히 넘겨주어야 합니다. 또한 취소 기능을 위해 이전 상태를 기억하거나 역연산을 수행하는 로직을 설계합니다.

### [연습 문제] 커맨드 패턴 구현

텍스트 편집기의 값 증가/감소 연산을 수행하고 이를 실행 취소(Undo)할 수 있는 커맨드 시스템을 구현하세요.

1. **리시버(Receiver):** `Editor` 구조체는 현재 텍스트의 크기(`value`) 상태를 가집니다.
2. **인터페이스:** `Command` 트레이트는 명령을 실행하는 `execute` 메서드와 이를 되돌리는 `undo` 메서드를 가집니다.
3. **구체적인 커맨드:** `IncrementCommand` 구조체는 내부에 `amount` 값을 가지며, `execute` 시 `Editor` 시스템의 `value`를 그만큼 증가시키고 `undo` 시 감소시킵니다.

```rust
// 1. 요청을 수신하여 실제 업무를 처리하는 객체 (Receiver)
struct Editor {
    value: i32,
}

impl Editor {
    fn new() -> Self {
        Editor { value: 0 }
    }
}

// 2. 커맨드 인터페이스 (Command)
trait Command {
    fn execute(&mut self, editor: &mut Editor);
    fn undo(&mut self, editor: &mut Editor);
}

// 3. 구체적인 커맨드 객체 (Concrete Command)
struct IncrementCommand {
    amount: i32,
}

impl IncrementCommand {
    fn new(amount: i32) -> Self {
        IncrementCommand { amount }
    }
}

// IncrementCommand에 Command 트레이트를 구현하세요.
// execute는 editor.value에 amount를 더합니다.
// undo는 editor.value에서 amount를 뺍니다.
impl Command for IncrementCommand {
    // 이 부분을 구현하세요.
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
```

[현재 단계: 행동 패턴 - 커맨드]