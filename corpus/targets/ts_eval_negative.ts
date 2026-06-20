function processExpression(expr: string) {
    const sanitized = expr.replace(/[^0-9+\-*/() ]/g, "");
    return Function(`"use strict"; return (${sanitized})`)();
}

function formatNumber(value: number): string {
    return value.toString();
}

function parseAmount(input: string): number {
    return Number(input);
}

function classNames(...classes: (string | undefined | false)[]): string {
    return classes.filter(Boolean).join(" ");
}

function toFixed(value: number, digits: number): string {
    return value.toFixed(digits);
}

function parseIntSafe(input: string, radix: number = 10): number {
    const result = parseInt(input, radix);
    return isNaN(result) ? 0 : result;
}

function parseFloatSafe(input: string): number {
    const result = parseFloat(input);
    return isNaN(result) ? 0 : result;
}
