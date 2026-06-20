function searchUsers(name: string) {
    const query = "SELECT * FROM users WHERE name = ?";
    db.query(query, [name]);
}
