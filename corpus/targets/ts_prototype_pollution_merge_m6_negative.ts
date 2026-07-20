// SAFE: Implements safe alternative
// SAFE: Filters out __proto__ and constructor keys before merging
function merge(target: any, source: any) {
    const safeKeys = Object.keys(source).filter(k => k !== "__proto__" && k !== "constructor");
    for (let key of safeKeys) { target[key] = source[key]; }
    return target;
}
function handlerA(target: any, source: any) { return merge(target, source); }
function handlerB(target: any, userInput: any) { return merge(target, userInput); }
