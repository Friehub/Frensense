# API Reference

## Programmatic Interface (Node.js)

### Class: GenSense
The primary entry point for the diagnostic engine.

#### Constructor: new GenSense(options)
- `options.environment`: String ('development' | 'production').
- `options.tags`: Array of strings (e.g., ['security', 'performance']).

#### method: auditContent(filePath, content)
Runs a semantic audit on a code string.
- Returns: Array of Advisory objects.

#### method: auditPath(directoryPath)
Runs a recursive audit on a filesystem path.
- Returns: Array of Advisory objects.
