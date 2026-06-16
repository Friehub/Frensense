const DANGEROUS_KEYS = new Set(["__proto__", "constructor", "prototype"]);

function mergeSettings(target: any, source: any) {
    for (const key in source) {
        if (DANGEROUS_KEYS.has(key)) continue;
        target[key] = source[key];
    }
    return target;
}

function deepMerge(base: any, override: any) {
    const filtered = Object.fromEntries(
        Object.entries(override).filter(([k]) => !DANGEROUS_KEYS.has(k))
    );
    Object.assign(base, filtered);
    return base;
}

function applyConfig(userInput: any) {
    const config: Record<string, unknown> = {};
    const key = String(userInput.field);
    if (DANGEROUS_KEYS.has(key)) throw new Error("Forbidden key");
    const value = userInput.value;
    config[key] = value;
    return config;
}
