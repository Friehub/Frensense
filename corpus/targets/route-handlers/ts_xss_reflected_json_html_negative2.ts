// SAFE: Server HTML-escapes all user fields before JSON serialization
import express from "express";

function escapeHtml(str: string): string {
    return str.replace(/[&<>"']/g, (c) => `&#${c.charCodeAt(0)};`);
}

export async function getUserProfile(req: express.Request, res: express.Response) {
    const user = await db.findUser(req.params.id);
    res.json({
        name: escapeHtml(user.name),
        bio: escapeHtml(user.bio),
        website: escapeHtml(user.website),
    });
}
