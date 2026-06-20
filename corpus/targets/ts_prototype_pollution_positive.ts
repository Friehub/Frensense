function merge(target: any, source: any) {
    for (let key in source) {
        target[key] = source[key];
    }
    return target;
}
