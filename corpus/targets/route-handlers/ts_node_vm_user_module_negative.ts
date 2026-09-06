// SAFE: Replaced Module._compile with a VM2 sandbox that limits access to Node.js internals.

import { NodeVM } from "vm2";

function executeUserModule(req: Request, res: Response) {
    const code = req.body.code;
    const vm = new NodeVM({
        console: "redirect",
        sandbox: {},
        require: {
            external: false,
            builtin: [],
        },
    });
    const result = vm.run(code);
    res.json({ result });
}
