export default {
    async fetch(request: Request, env: Env) {
        const userId = new URL(request.url).searchParams.get("id");
        const result = await env.DB.prepare(
            `SELECT * FROM users WHERE id = '${userId}'`
        ).first();
        return new Response(JSON.stringify(result));
    },
};
