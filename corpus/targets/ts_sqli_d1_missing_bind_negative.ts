export default {
    async fetch(request: Request, env: Env) {
        const name = new URL(request.url).searchParams.get("name");
        const result = await env.DB.prepare(
            "SELECT * FROM users WHERE name = ?"
        ).bind(name).first();
        return new Response(JSON.stringify(result));
    },
};
