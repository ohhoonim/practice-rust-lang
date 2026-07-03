// 주어진 문자열 `s`가 괄호 `'('`, `')'`, `'{'`, `'}'`, `'['`, `']'`만으로 이루어져 있을 때, 올바른 괄호 문자열인지 판별하는 `is_valid` 함수를 완성하세요.
// **올바른 괄호의 조건:**
// 1. 열린 괄호는 반드시 같은 타입의 닫힌 괄호로 닫혀야 합니다.
// 2. 열린 괄호는 올바른 순서대로 닫혀야 합니다.
fn is_valid(s: &str) -> bool {
    let mut stack = Vec::new();

    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            ')' | ']' | '}' => {
                if stack.pop() != Some(c) {
                    return false;
                }
            }
            _ => {}
        }
    }

    stack.is_empty()
}


// 두 개의 스택(Rust의 Vec)을 사용하여 선입선출(FIFO) 인터페이스를 지원하는 큐 MyQueue를 구현하세요.
struct MyQueue {
    stack_in: Vec<i32>,
    stack_out: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            stack_in: Vec::new(),
            stack_out: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.stack_in.push(x);
    }

    fn pop(&mut self) -> Option<i32> {
        self.peek();
        self.stack_out.pop()
    }

    fn peek(&mut self) -> Option<i32> {
        if self.stack_out.is_empty() {
            while let Some(x) = self.stack_in.pop() {
                self.stack_out.push(x);
            }
        }
        self.stack_out.last().copied()
    }

    fn empty(&self) -> bool {
        self.stack_in.is_empty() && self.stack_out.is_empty()
    }
}

#[cfg(test)]
#[path = "data_structure_test.rs"]
mod data_structure_test;
