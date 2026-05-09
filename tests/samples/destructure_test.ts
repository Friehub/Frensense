function testDestructure() {
    const { password, email } = req.body; // password should be tainted
    console.log("Leak 1:", password);
}

function testAlias() {
    const { password: p } = req.body; // 'p' should be tainted
    console.log("Leak 2:", p);
}

function testArray() {
    const [token, salt] = getCreds(); // 'token' should be tainted
    console.log("Leak 3:", token);
}

function getCreds() { return ["secret_token", "salt"]; }
