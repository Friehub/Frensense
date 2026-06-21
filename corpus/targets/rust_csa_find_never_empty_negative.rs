use std::collections::HashMap;

#[derive(Clone)]
struct User {
    id: u64,
    name: String,
    email: String,
    status: String,
}

struct SearchOptions {
    include_deleted: bool,
}

struct UserRepository {
    cache: HashMap<u64, User>,
}

impl UserRepository {
    fn new() -> Self {
        UserRepository {
            cache: HashMap::new(),
        }
    }

    fn query_database(&self, _id: u64) -> Vec<User> {
        Vec::new()
    }

    fn find(&self, id: u64, options: &SearchOptions) -> Option<User> {
        if id == 0 {
            return None;
        }

        if let Some(cached) = self.cache.get(&id) {
            if cached.status == "deleted" && !options.include_deleted {
                return None;
            }
            return Some(cached.clone());
        }

        let results = self.query_database(id);
        let user = results.into_iter().next()?;

        if user.status == "deleted" && !options.include_deleted {
            return None;
        }

        Some(user)
    }
}
