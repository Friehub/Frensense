// SAFE: Used a fixed Lua script with KEYS/ARGV parameters, never interpolating user input into the script string.

import { createClient } from "redis";

const redis = createClient();

const COMPARE_AND_SET_SCRIPT = `
    local current = redis.call("GET", KEYS[1])
    if current then
        redis.call("SET", KEYS[1], ARGV[1])
        return 1
    end
    return 0
`;

async function atomicConditionalUpdate(req: Request, res: Response) {
    const key = req.params.key;
    const value = req.body.value;
    const result = await redis.eval(COMPARE_AND_SET_SCRIPT, {
        keys: [key],
        arguments: [JSON.stringify(value)],
    });
    res.json({ updated: result === 1 });
}
