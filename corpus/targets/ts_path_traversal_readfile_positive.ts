function readFile(filename: string) {
    const data = fs.readFileSync(filename, "utf8");
    return data;
}
