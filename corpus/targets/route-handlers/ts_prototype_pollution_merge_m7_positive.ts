// [frensense]
// observation: A merge operation copies properties from a user-controlled source object without filtering dangerous keys via destructured object property.
// impact: An attacker can set __proto__ or constructor.prototype properties to pollute all objects' prototypes
// improvement: Filter out __proto__ and constructor keys, or use Object.assign with null-prototype target

async function handlerA(req: Request, res: Response) {
    const { input } = req.query;
    for (let key in input) { target[key] = input[key]; } return target;
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const { value } = req.body;
    for (let key in value) { target[key] = value[key]; } return target;
    res.json({ ok: true });
}
