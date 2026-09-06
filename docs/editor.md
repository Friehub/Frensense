# Editor Integration

Frensense does not yet ship a dedicated editor extension. The following patterns integrate it into existing development workflows using tools that are already available.

---

## VS Code

### Option 1: Task Runner

Add a task to `.vscode/tasks.json`. This lets you invoke Frensense on demand from the Command Palette or `Ctrl+Shift+B`.

Create or update `.vscode/tasks.json` in your project root:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Frensense: Audit Project",
      "type": "shell",
      "command": "npx @friehub/frensense audit ${workspaceFolder}",
      "group": {
        "kind": "build",
        "isDefault": true
      },
      "presentation": {
        "reveal": "always",
        "panel": "shared",
        "clear": true
      },
      "problemMatcher": []
    },
    {
      "label": "Frensense: Audit Current File",
      "type": "shell",
      "command": "npx @friehub/frensense audit ${file}",
      "group": "build",
      "presentation": {
        "reveal": "always",
        "panel": "shared",
        "clear": true
      },
      "problemMatcher": []
    }
  ]
}
```

Run via: **Terminal > Run Task** or `Ctrl+Shift+B`.

---

### Option 2: On-Save Integration

Install the [Run on Save](https://marketplace.visualstudio.com/items?itemName=emeraldwalk.RunOnSave) VS Code extension, then add this to your `settings.json`:

```json
{
  "emeraldwalk.runonsave": {
    "commands": [
      {
        "match": "\\.(rs|ts|js|sol)$",
        "cmd": "npx @friehub/frensense audit ${file}",
        "isAsync": true
      }
    ]
  }
}
```

This runs a file-level audit every time you save a supported file. Output appears in the Run on Save output channel.

---

### Option 3: Workspace Recommended Extensions

To ensure all contributors on a project can quickly set up editor integration, add a `.vscode/extensions.json` file:

```json
{
  "recommendations": [
    "emeraldwalk.RunOnSave"
  ]
}
```

This will prompt new contributors to install the recommended extension when they open the workspace.

---

## Pre-Commit Hook

Integrate Frensense into the git commit lifecycle so no findings reach the repository.

### Using Husky (Node.js projects)

```bash
npm install --save-dev husky
npx husky init
echo "npx @friehub/frensense audit . --tag security" > .husky/pre-commit
```

### Using a Shell Script (any project)

Create `.git/hooks/pre-commit`:

```bash
#!/bin/sh
echo "Running Frensense audit..."
npx @friehub/frensense audit . --tag security

if [ $? -ne 0 ]; then
  echo "Frensense found issues. Commit blocked."
  exit 1
fi
```

Make it executable:

```bash
chmod +x .git/hooks/pre-commit
```

---

## CI / GitHub Actions

Add Frensense as a quality gate in your CI pipeline. The process exits with code `1` if any findings are produced.

### Audit on Pull Request

```yaml
name: Code Quality

on:
  pull_request:
    branches: [main]

jobs:
  frensense:
    name: Semantic Audit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: 20

      - name: Install Frensense
        run: npm install -g @friehub/frensense

      - name: Run Audit
        run: frensense audit . --tag security --tag reliability
```

### Fail Only on Critical Findings

If you want to block CI only on `Critical` severity findings, you can pipe output and filter:

```bash
frensense audit . | grep -q "Critical" && exit 1 || exit 0
```

---

## JetBrains IDEs (RustRover, WebStorm)

Use the built-in **External Tools** feature:

1. Go to **Settings > Tools > External Tools**.
2. Click **+** to add a new tool.
3. Configure:
   - **Name**: Frensense Audit
   - **Program**: `npx`
   - **Arguments**: `@friehub/frensense audit $FilePath$`
   - **Working Directory**: `$ProjectFileDir$`
4. Optionally bind it to a keyboard shortcut via **Keymap**.
