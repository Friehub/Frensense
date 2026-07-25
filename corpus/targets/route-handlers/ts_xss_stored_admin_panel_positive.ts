// [frensense]
// observation: User data displayed in an admin panel — name, email, activity logs — is rendered without HTML encoding, making admin users vulnerable to stored XSS.
// impact: An attacker with a low-privilege account stores a payload in their profile or activity data, which executes when an admin views the management panel, leading to account takeover.
// improvement: Encode all user-supplied data anywhere it appears in admin UIs, as admin sessions are high-value targets.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss

import express from "express";

export async function adminUserList(req: express.Request, res: express.Response) {
    const users = await db.query("SELECT id, username, email, role FROM users");
    let html = "<table><tr><th>ID</th><th>Username</th><th>Email</th><th>Role</th></tr>";
    for (const u of users) {
        html += `<tr><td>${u.id}</td><td>${u.username}</td><td>${u.email}</td><td>${u.role}</td></tr>`;
    }
    html += "</table>";
    res.send(html);
}

export async function adminUserDetail(req: express.Request, res: express.Response) {
    const user = await db.query("SELECT * FROM users WHERE id = ?", [req.params.id]);
    const logs = await db.query("SELECT action, details FROM audit_log WHERE user_id = ?", [req.params.id]);
    res.send(`
        <h1>${user[0].username}</h1>
        <p>Email: ${user[0].email}</p>
        <p>Bio: ${user[0].bio}</p>
        <h2>Activity Log</h2>
        <ul>${logs.map((l: any) => `<li>${l.action}: ${l.details}</li>`).join("")}</ul>
    `);
}
