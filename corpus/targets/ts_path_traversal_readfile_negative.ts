function readFile(filename: string) {
    const safePath = path.join("/uploads", path.basename(filename));
    const data = fs.readFileSync(safePath, "utf8");
    return data;
}
