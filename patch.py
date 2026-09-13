with open('.github/workflows/release.yml', 'r') as f:
    content = f.read()

replacement = """      - name: Test Genesense NPM Wrapper before publish
        run: |
          cd genesense-npm
          PACKAGE=$(npm pack --silent 2>&1 | tail -1)
          echo "Genesense wrapper package: $PACKAGE"
          TESTDIR=$(mktemp -d)
          cd "$TESTDIR"
          npm init -y > /dev/null
          npm install "$GITHUB_WORKSPACE/genesense-npm/$PACKAGE" > /dev/null 2>&1
          npx genesense --version
          echo "fn main() { let y = 2; }" > test2.rs
          npx genesense test2.rs --json > /dev/null
          echo "Genesense NPM wrapper OK"

      - name: Publish to NPM"""

content = content.replace("      - name: Publish to NPM", replacement, 1)

with open('.github/workflows/release.yml', 'w') as f:
    f.write(content)
