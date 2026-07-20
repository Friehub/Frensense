// SAFE: Uses a template engine (EJS) with auto-escaping
import express from "express";

export async function viewProfile(req: express.Request, res: express.Response) {
    const user = await db.query("SELECT username, bio, website FROM users WHERE id = ?", [req.params.id]);
    res.render("profile", { user: user[0] });
}
