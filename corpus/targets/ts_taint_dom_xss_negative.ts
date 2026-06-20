function handleRequest(req: Request) {
    const userInput = req.query.name;
    const safe = DOMPurify.sanitize(userInput);
    document.getElementById("output").textContent = safe;
}
