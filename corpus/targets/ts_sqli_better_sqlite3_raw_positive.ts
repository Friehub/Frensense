import Database from "better-sqlite3";
const db = new Database("app.db");

function getUser(userId: string) {
    const stmt = db.prepare(`SELECT * FROM users WHERE id = '${userId}'`);
    return stmt.get();
}
