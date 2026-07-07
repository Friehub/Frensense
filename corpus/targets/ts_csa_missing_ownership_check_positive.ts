// [frensense]
// observation: The handler fetches a resource (e.g., project, file, or setting) using only a client-supplied ID, without verifying that the authenticated user actually owns that resource.
// impact: This results in an Insecure Direct Object Reference (IDOR) vulnerability. Any authenticated user can read, modify, or delete another customer's data simply by supplying a different ID in the request.
// improvement: Precede the resource fetch with an ownership verification check. Ensure the resource's owner_id matches the session's customerId before granting access.

export async function handleWorkspaceReadFile(req: any, env: any) {
    const projectId = req.query.projectId;
    const path = req.query.path;
    
    // Vulnerable: no ownership check on projectId
    const file = await env.DB.prepare("SELECT * FROM files WHERE project_id = ? AND path = ?")
        .bind(projectId, path)
        .first();
        
    if (!file) return new Response("Not found", { status: 404 });
    return new Response(file.content);
}
