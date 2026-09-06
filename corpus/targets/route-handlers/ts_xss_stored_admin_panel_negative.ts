// SAFE: All user data in the admin panel is HTML-escaped
import express from "express";

function escapeHtml(str: string): string {
    return str.replace(/[&<>"']/g, (c) => `&#${c.charCodeAt(0)};`);
}

export async function adminUserList(req: express.Request, res: express.Response) {
    const users = await db.query("SELECT id, username, email, role FROM users");
    let html = "<table><tr><th>ID</th><th>Username</th><th>Email</th><th>Role</th></tr>";
    for (const u of users) {
        html += `<tr><td>${escapeHtml(String(u.id))}</td><td>${escapeHtml(u.username)}</td><td>${escapeHtml(u.email)}</td><td>${escapeHtml(u.role)}</td></tr>`;
    }
    html += "</table>";
    res.send(html);
}

export async function adminUserDetail(req: express.Request, res: express.Response) {
    const user = await db.query("SELECT * FROM users WHERE id = ?", [req.params.id]);
    const logs = await db.query("SELECT action, details FROM audit_log WHERE user_id = ?", [req.params.id]);
    res.send(`
        <h1>${escapeHtml(user[0].username)}</h1>
        <p>Email: ${escapeHtml(user[0].email)}</p>
        <p>Bio: ${escapeHtml(user[0].bio)}</p>
        <h2>Activity Log</h2>
        <ul>${logs.map((l: any) => `<li>${escapeHtml(l.action)}: ${escapeHtml(l.details)}</li>`).join("")}</ul>
    `);
}
