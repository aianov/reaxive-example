use dioxus::prelude::*;
use reaxive::prelude::*;

#[derive(Clone, Debug)]
pub struct User {
    pub name: String,
    pub last_name: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: "Иван".to_string(),
            last_name: "Петров".to_string(),
        }
    }
}

reaxive_store!(CounterStore {
    count: i32 = 0,
    user: User = User::default()
});

impl CounterStore {
    pub fn increment(&self) {
        self.count.set(|count| *count += 1);
    }

    pub fn decrement(&self) {
        self.count.set(|count| *count -= 1);
    }

    pub fn get_count(&self) -> i32 {
        self.count.get()
    }

    pub fn set_name(&self, name: String) {
        self.user.set(|user| user.name = name);
    }

    pub fn set_last_name(&self, last_name: String) {
        self.user.set(|user| user.last_name = last_name);
    }

    pub fn get_user(&self) -> User {
        self.user.get()
    }

    pub fn get_full_name(&self) -> String {
        let user = self.user.get();
        format!("{} {}", user.name, user.last_name)
    }

    pub fn get_initials(&self) -> String {
        let user = self.user.get();
        format!(
            "{}.{}",
            user.name.chars().next().unwrap_or('?'),
            user.last_name.chars().next().unwrap_or('?')
        )
    }

    pub fn reset_user(&self) {
        self.user.assign(User::default());
    }
}
