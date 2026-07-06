// option chaining

use std::{cell::RefCell, num::ParseIntError};

struct User {
    name: String,
    score: Option<i32>,
}
fn get_bonus_score(user: &User) -> i32 {
    user.score.map(|s| s * 2).unwrap_or(0)
}

// Result 에러 전파
fn square_number(num_str: &str) -> Result<i32, ParseIntError> {
    let num = num_str.parse::<i32>()?;
    Ok(num * num)
}

// RefCell

struct MockLogger {
    msg_count: RefCell<u32>,
}
impl MockLogger {
    fn new() -> Self {
        MockLogger { msg_count: RefCell::new(0) }
    }

    fn send_log(&self, _msg: &str) {
        *self.msg_count.borrow_mut() += 1;
    }

    fn get_count(&self) -> u32 {
        *self.msg_count.borrow()
    }
    
}


#[cfg(test)]
#[path = "smart_pointer_test.rs"]
mod smart_pointer_test;
