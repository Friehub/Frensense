function merge(target: any, source: any) {
    const safeKeys = Object.keys(source).filter(k => k !== "__proto__" && k !== "constructor");
    for (let key of safeKeys) {
        target[key] = source[key];
    }
    return target;
}
