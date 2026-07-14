import { Hono } from "hono";

const app = new Hono();

app.post('/stop', async (c) => {
  const body = await c.req.json();
  const wf = c.env.AGENT_RUN_WORKFLOW;
  const session = c.get("session");
  
  if (wf && body.run_id) {
    const inst = await wf.get(body.run_id);
    const status = await inst.status();
    
    // SAFE: Verifying ownership before mutating
    if (status.output?._customerId !== session.customerId) {
      return c.json({ error: "forbidden" }, 403);
    }
    
    await inst.pause();
    
    return c.json({ status: "paused" });
  }
  
  return c.json({ error: "missing run_id" }, 400);
});
