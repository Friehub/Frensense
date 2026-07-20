// [frensense]
// observation: A merge operation copies properties from a user-controlled source object without filtering dangerous keys through a helper function.
// impact: An attacker can set __proto__ or constructor.prototype properties to pollute all objects' prototypes
// improvement: Filter out __proto__ and constructor keys, or use Object.assign with null-prototype target

function getValue(input: string): string {
    return input;
}

function prepareInput(raw: any): string {
    return raw;
}

async function handlerA(req: Request, res: Response) {
    const val = getValue(source);
    for (let key in val) { target[key] = val[key]; } return target;
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const val = prepareInput(userInput);
    for (let key in val) { target[key] = val[key]; } return target;
    res.json({ ok: true });
}
