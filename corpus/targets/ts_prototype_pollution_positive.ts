function merge(target: any, source: any) {
    target.__proto__ = source;
}
