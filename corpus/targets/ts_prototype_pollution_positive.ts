function mergeSettings(target: any, source: any) {
    for (const key in source) {
        target[key] = source[key];
    }
    return target;
}

function deepMerge(base: any, override: any) {
    Object.assign(base, override);
    return base;
}

function applyConfig(userInput: any) {
    const config = {};
    const key = userInput.field;
    const value = userInput.value;
    config[key] = value;
    return config;
}
