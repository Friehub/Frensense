function handleUserAuth() {
    const userPassword = req.body.password; // SOURCE: Born tainted
    
    // 1. First hop
    const intermediate = userPassword; 
    
    // 2. Second hop (obfuscation)
    const finalRef = intermediate; 
    
    // 3. SINK: The leak!
    console.log("DEBUG: Final auth state is", finalRef); 
}

function safeFunction() {
    const name = "Alice";
    console.log("Hello", name); // Should NOT be flagged
}
