// Rule: TS_PATH_TRAVERSAL (negative — no rule expected)
function readFile() {
    fs.readFileSync("/etc/config.json"); // Hardcoded path — safe
}
