// [frensense]
// observation: A merge operation copies properties from a user-controlled source object without filtering dangerous keys via an array element access.
// impact: An attacker can set __proto__ or constructor.prototype properties to pollute all objects' prototypes
// improvement: Filter out __proto__ and constructor keys, or use Object.assign with null-prototype target
// cwe: CWE-1321
// cvss: 9.8
// owasp: A03:2021
// severity: Critical

async function handlerA(req: Request, res: Response) {
    const arr = [source];
    for (let key in arr[0]) { target[key] = arr[0][key]; } return target;
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const items = [userInput];
    for (let key in items[0]) { target[key] = items[0][key]; } return target;
    res.json({ ok: true });
}
