// [frensense]
// observation: A merge operation copies properties from a user-controlled source object without filtering dangerous keys via a template literal interpolation.
// impact: An attacker can set __proto__ or constructor.prototype properties to pollute all objects' prototypes
// improvement: Filter out __proto__ and constructor keys, or use Object.assign with null-prototype target

async function handlerA(req: Request, res: Response) {
    for (let key in source) { target[key] = source[key]; } return target;
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    for (let key in userInput) { target[key] = userInput[key]; } return target;
    res.json({ ok: true });
}
