// [frensense]
// observation: User-controlled input is passed directly to Redis EVAL as a Lua script, allowing arbitrary server-side command execution.
// impact: An attacker can inject malicious Lua code that reads or modifies any key in the Redis database, bypassing access controls and potentially compromising the entire cache layer.
// improvement: Never pass user input to EVAL/EVALSHA as a script. Use parameterized Redis commands or pass user values as KEYS/ARGV to the script instead.

import { createClient } from "redis";

const redis = createClient();

async function runCustomScript(req: Request, res: Response) {
    const userScript = req.body.script;
    const result = await redis.eval(userScript, { keys: [], arguments: [] });
    res.json({ result });
}

async function conditionalUpdate(req: Request, res: Response) {
    const key = req.params.key;
    const condition = req.body.condition;
    const script = `if redis.call("GET", "${key}") ${condition} then return redis.call("SET", "${key}", ${req.body.value}) end`;
    const result = await redis.eval(script, { keys: [], arguments: [] });
    res.json({ result });
}
