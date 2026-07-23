// SAFE: Admin panel uses a templating engine with auto-escaping (Handlebars)
import express from "express";

export async function adminUserList(req: express.Request, res: express.Response) {
    const users = await db.query("SELECT id, username, email, role FROM users");
    res.render("admin/users", { users });
}

export async function adminUserDetail(req: express.Request, res: express.Response) {
    const user = await db.query("SELECT * FROM users WHERE id = ?", [req.params.id]);
    const logs = await db.query("SELECT action, details FROM audit_log WHERE user_id = ?", [req.params.id]);
    res.render("admin/user-detail", { user: user[0], logs });
}
