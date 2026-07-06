
// Vec 데이터 조작

use std::collections::{HashMap, HashSet};

struct TodoList {
    tasks: Vec<String>,
}
impl TodoList {
    fn new() -> Self {
        TodoList {
            tasks: Vec::new(),
        }
    }

    fn add_task(&mut self, task: String) {
        self.tasks.push(task);
    }

    fn complete_task(&mut self, index: usize) -> String {
        self.tasks.remove(index)
    }
}

// HashMap Entry API

fn count_words(words: Vec<String>) -> HashMap<String, u32> {
    let mut frequencies = HashMap::new();

    for word in words {
        *frequencies.entry(word).or_insert(0) += 1;
    }

    frequencies
}


// HashSet

struct UserManager {
    users: HashSet<u32>,
}
impl UserManager {
    fn new() -> Self {
        UserManager {
            users: HashSet::new(),
        }
    }

    fn register_user(&mut self, user_id: u32) -> bool {
        self.users.insert(user_id)
    }

    fn total_unique_users(&self) -> usize {
        self.users.len()
    }
}



#[cfg(test)]
#[path="collection_data_structure_test.rs"]
mod collection_data_structure_test;