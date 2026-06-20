function handleRequest(req: Request) {
    const userInput = req.query.name;
    document.getElementById("output").innerHTML = userInput;
}
