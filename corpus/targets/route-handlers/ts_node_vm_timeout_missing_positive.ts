// [frensense]
// observation: vm.runInNewContext is called with user-controlled code without specifying a timeout, allowing an attacker to execute an infinite loop and cause a Denial of Service.
// impact: A malicious script can run indefinitely, blocking the event loop and consuming CPU, leading to application unavailability.
// improvement: Always specify a timeout parameter when executing untrusted code with vm.runInNewContext or vm.Script.

import vm from "node:vm";

function executeUserCode(req: Request, res: Response) {
    const userCode = req.body.code;
    const sandbox = { data: req.body.input };
    const result = vm.runInNewContext(userCode, sandbox);
    res.json({ result });
}
