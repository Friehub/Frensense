import { Hono } from "hono";

const app = new Hono();

app.post('/stop', async (c) => {
  const body = await c.req.json();
  const wf = c.env.AGENT_RUN_WORKFLOW;
  
  if (wf && body.run_id) {
    const inst = await wf.get(body.run_id);
    
    // VULNERABILITY: IDOR - mutating workflow without checking owner
    // Any authenticated user can pause/resume ANY run_id
    await inst.pause();
    
    return c.json({ status: "paused" });
  }
  
  return c.json({ error: "missing run_id" }, 400);
});
