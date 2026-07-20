// SAFE: Returns a Result type with distinct error variants for each not-found case
use std::collections::HashMap;

#[derive(Debug)]
enum FindError {
    InvalidId,
    NotFound,
    Deleted,
}

fn find_user(id: u64, options: &SearchOptions, cache: &HashMap<u64, User>) -> Result<User, FindError> {
    if id == 0 {
        return Err(FindError::InvalidId);
    }

    if let Some(cached) = cache.get(&id) {
        if cached.status == "deleted" && !options.include_deleted {
            return Err(FindError::Deleted);
        }
        return Ok(cached.clone());
    }

    let results = query_database(id);
    let user = results.into_iter().next().ok_or(FindError::NotFound)?;

    if user.status == "deleted" && !options.include_deleted {
        return Err(FindError::Deleted);
    }

    Ok(user)
}
