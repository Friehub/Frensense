// SAFE: A timeout is specified for vm.runInNewContext to prevent infinite loops.

import vm from "node:vm";

function executeUserCode(req: Request, res: Response) {
    const userCode = req.body.code;
    const sandbox = { data: req.body.input };
    const result = vm.runInNewContext(userCode, sandbox, { timeout: 1000 });
    res.json({ result });
}
