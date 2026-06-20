function searchUsers(name: string) {
    const query = `SELECT * FROM users WHERE name = '${name}'`;
    db.query(query);
}
