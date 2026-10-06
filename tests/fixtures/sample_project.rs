//! Sample project fixtures for testing

pub const SAMPLE_RUST_CODE: &str = r#"
use std::collections::HashMap;

/// A user in the system
pub struct User {
    id: u64,
    name: String,
    email: String,
}

impl User {
    /// Create a new user
    pub fn new(id: u64, name: String, email: String) -> Self {
        Self { id, name, email }
    }
    
    /// Get the user's name
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// User repository
pub struct UserRepository {
    users: HashMap<u64, User>,
}

impl UserRepository {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }
    
    pub fn add(&mut self, user: User) {
        self.users.insert(user.id, user);
    }
    
    pub fn get(&self, id: u64) -> Option<&User> {
        self.users.get(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_user_creation() {
        let user = User::new(1, "Alice".to_string(), "alice@example.com".to_string());
        assert_eq!(user.name(), "Alice");
    }
}
"#;

pub const SAMPLE_PYTHON_CODE: &str = r#"
class User:
    """A user in the system"""
    
    def __init__(self, id: int, name: str, email: str):
        self.id = id
        self.name = name
        self.email = email
    
    def get_name(self) -> str:
        return self.name


class UserRepository:
    """User repository"""
    
    def __init__(self):
        self.users = {}
    
    def add(self, user: User):
        self.users[user.id] = user
    
    def get(self, id: int) -> User:
        return self.users.get(id)


def main():
    repo = UserRepository()
    user = User(1, "Alice", "alice@example.com")
    repo.add(user)
    print(repo.get(1).get_name())


if __name__ == "__main__":
    main()
"#;

pub const SAMPLE_TYPESCRIPT_CODE: &str = r#"
interface User {
    id: number;
    name: string;
    email: string;
}

class UserRepository {
    private users: Map<number, User> = new Map();
    
    add(user: User): void {
        this.users.set(user.id, user);
    }
    
    get(id: number): User | undefined {
        return this.users.get(id);
    }
}

function createUser(id: number, name: string, email: string): User {
    return { id, name, email };
}

export { User, UserRepository, createUser };
"#;
