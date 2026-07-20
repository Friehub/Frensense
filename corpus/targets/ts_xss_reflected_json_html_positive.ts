// [frensense]
// observation: A JSON API response contains user-controlled data that is later rendered as HTML in a client-side template, without sanitization at either the API or the client.
// impact: Stored or reflected XSS via API responses — a server returns user input as JSON, and the client renders it via innerHTML.
// improvement: Sanitize output at the rendering layer, or encode data in the API response.

import express from "express";

export async function getUserProfile(req: express.Request, res: express.Response) {
    const user = await db.findUser(req.params.id);
    res.json({ name: user.name, bio: user.bio, website: user.website });
}

export function renderProfile() {
    fetch("/api/user/1")
        .then(r => r.json())
        .then(user => {
            document.getElementById("profile")!.innerHTML = `
                <h2>${user.name}</h2>
                <p>${user.bio}</p>
                <a href="${user.website}">Website</a>
            `;
        });
}
