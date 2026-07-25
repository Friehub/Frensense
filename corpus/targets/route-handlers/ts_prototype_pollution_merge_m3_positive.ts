// [frensense]
// observation: A merge operation copies properties from a user-controlled source object without filtering dangerous keys through multiple variable assignments.
// impact: An attacker can set __proto__ or constructor.prototype properties to pollute all objects' prototypes
// improvement: Filter out __proto__ and constructor keys, or use Object.assign with null-prototype target
// cwe: CWE-1321
// cvss: 9.8
// owasp: A03:2021
// severity: Critical

async function handlerA(req: Request, res: Response) {
    const a = source;
    const b = a;
    for (let key in b) { target[key] = b[key]; } return target;
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const x = userInput;
    const y = x;
    const z = y;
    for (let key in z) { target[key] = z[key]; } return target;
    res.json({ ok: true });
}
