// [frensense]
// observation: A merge operation copies properties from a user-controlled source object without filtering dangerous keys via string concatenation.
// impact: An attacker can set __proto__ or constructor.prototype properties to pollute all objects' prototypes
// improvement: Filter out __proto__ and constructor keys, or use Object.assign with null-prototype target

async function handlerA(req: Request, res: Response) {
    const keys = Object.keys(source); for (let key of keys) { target[key] = source[key]; } return target;
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const keys = Object.keys(userInput); for (let key of keys) { target[key] = userInput[key]; } return target;
    res.json({ ok: true });
}
