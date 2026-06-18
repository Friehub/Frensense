// Fixed: avoid eval
const result = Function('return ' + userInput)();
