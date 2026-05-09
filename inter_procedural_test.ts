function sinkWrapper(sensitiveData) {
    // The Auditor should trace 'sensitiveData' here
    console.log("Internal System Log:", sensitiveData);
}

function handleLogin() {
    const password = req.body.password; // SOURCE: Tainted!
    
    // The jump!
    sinkWrapper(password);
}

function handleNormal() {
    const username = "guest_user";
    sinkWrapper(username); // SAFE
}
