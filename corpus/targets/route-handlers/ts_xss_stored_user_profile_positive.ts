// [frensense]
// observation: User profile fields (name, bio, website URL) are displayed on the profile page without HTML encoding, allowing stored XSS via profile data.
// impact: An attacker updates their profile with malicious JavaScript. Every visitor to their profile executes the payload, harvesting cookies or performing actions as the victim.
// improvement: Encode all user profile fields on output, especially free-text fields like bio.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss

import express from "express";

export async function viewProfile(req: express.Request, res: express.Response) {
    const user = await db.query("SELECT username, bio, website FROM users WHERE id = ?", [req.params.id]);
    const u = user[0];
    res.send(`
        <div class="profile">
            <h2>${u.username}</h2>
            <p class="bio">${u.bio}</p>
            <a href="${u.website}" class="website">Visit website</a>
        </div>
    `);
}
