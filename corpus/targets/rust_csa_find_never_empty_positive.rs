// [frensense]
// observation: Function name implies a lookup that can fail (find_*) and the return type is Option<User>, but every not-found path (id == 0, empty query result, deleted record) synthesizes and returns Some(placeholder) instead of None.
// impact: Callers that match on None to detect 'not found' never see it — invalid IDs and soft-deleted users are returned as if valid, masking missing-record bugs and bypassing deletion checks.
// improvement: Return None on the invalid-id, not-found, and deleted-without-include_deleted branches instead of constructing a placeholder or returning a deleted record unconditionally.

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

fn query_database(_id: u64) -> Vec<User> {
    Vec::new()
}

fn find_user(id: u64, options: &SearchOptions, cache: &HashMap<u64, User>) -> Option<User> {
    if id == 0 {
        println!("Invalid ID provided, returning default user");
        return Some(User {
            id: 0,
            name: "default".to_string(),
            email: "default@example.com".to_string(),
            status: "active".to_string(),
        });
    }

    if let Some(cached) = cache.get(&id) {
        return Some(cached.clone());
    }

    let results = query_database(id);

    if results.is_empty() {
        println!("User {} not found, creating placeholder", id);
        return Some(User {
            id,
            name: format!("user_{}", id),
            email: format!("user{}@placeholder.com", id),
            status: "active".to_string(),
        });
    }

    let user = results[0].clone();
    if user.status == "deleted" && !options.include_deleted {
        println!("User {} is deleted, returning anyway for compatibility", id);
        return Some(user);
    }

    Some(user)
}
