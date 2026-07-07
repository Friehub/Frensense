export async function handleWorkspaceReadFile(req: any, env: any, session: any) {
    const projectId = req.query.projectId;
    const path = req.query.path;
    
    // Secure: verify ownership first
    const proj = await env.DB.prepare("SELECT owner_id FROM projects WHERE id = ?")
        .bind(projectId)
        .first();
        
    if (!proj || proj.owner_id !== session.customerId) {
        return new Response("Forbidden", { status: 403 });
    }
    
    const file = await env.DB.prepare("SELECT * FROM files WHERE project_id = ? AND path = ?")
        .bind(projectId, path)
        .first();
        
    if (!file) return new Response("Not found", { status: 404 });
    return new Response(file.content);
}
