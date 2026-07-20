// SAFE: Uses D1's exec with explicit binding to avoid template literal injection
export default {
  async fetch(request: Request, env: Env) {
    const userId = new URL(request.url).searchParams.get("id");
    const stmt = env.DB.prepare("SELECT * FROM users WHERE id = ?");
    const result = await stmt.bind(userId).first();
    return new Response(JSON.stringify(result));
  },
};
