// SAFE: Profile fields are HTML-escaped before rendering
import express from "express";

function escapeHtml(str: string): string {
    return str.replace(/[&<>"']/g, (c) => `&#${c.charCodeAt(0)};`);
}

export async function viewProfile(req: express.Request, res: express.Response) {
    const user = await db.query("SELECT username, bio, website FROM users WHERE id = ?", [req.params.id]);
    const u = user[0];
    res.send(`
        <div class="profile">
            <h2>${escapeHtml(u.username)}</h2>
            <p class="bio">${escapeHtml(u.bio)}</p>
            <a href="${escapeHtml(u.website)}" class="website">Visit website</a>
        </div>
    `);
}
