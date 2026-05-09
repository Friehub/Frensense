#!/bin/bash
# TaaS Hook Installer
# Links local hooks to the .git directory.

set -e

HOOK_SOURCE="hooks/pre-commit"
HOOK_TARGET=".git/hooks/pre-commit"

if [ ! -d ".git" ]; then
    echo "[ERROR] Not a git repository. Run 'git init' first."
    exit 1
fi

echo "[LINK] Linking $HOOK_SOURCE to $HOOK_TARGET..."
cp "$HOOK_SOURCE" "$HOOK_TARGET"
chmod +x "$HOOK_TARGET"

echo "[SUCCESS] Git hooks installed successfully."
