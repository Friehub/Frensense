// Vulnerable: XSS via innerHTML
document.getElementById('output').innerHTML = userInput;
