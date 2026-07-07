export async function getProjectData(req: any, env: any) {
    const id = req.query.id;
    // Missing ownership check, just returns data
    const data = await env.DB.prepare("SELECT * FROM projects_data WHERE p_id = ?").bind(id).first();
    return data;
}
