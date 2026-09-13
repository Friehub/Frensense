import re

with open('.github/workflows/release.yml', 'r') as f:
    text = f.read()

new_steps = """      - name: Test Genesense NPM Wrapper before publish
        run: |
          npm pack
          cd genesense-npm
          npm pack
          TESTDIR=$(mktemp -d)
          cd "$TESTDIR"
          npm init -y > /dev/null
          npm install "$GITHUB_WORKSPACE"/friehub-frensense-*.tgz
          npm install "$GITHUB_WORKSPACE"/genesense-npm/genesense-*.tgz
          npx genesense --version
          echo "fn main() { let y = 2; }" > test2.rs
          npx genesense test2.rs --json > /dev/null
          echo "Genesense NPM wrapper OK"
"""

text = re.sub(
r'''      - name: Test Genesense NPM Wrapper before publish
        run: \|
          MAIN_PACKAGE=\$\(npm pack --silent 2>&1 \| tail -1\)
          cd genesense-npm
          WRAPPER_PACKAGE=\$\(npm pack --silent 2>&1 \| tail -1\)
          TESTDIR=\$\(mktemp -d\)
          cd "\$TESTDIR"
          npm init -y > /dev/null
          npm install "\$GITHUB_WORKSPACE/\$MAIN_PACKAGE" > /dev/null 2>&1
          npm install "\$GITHUB_WORKSPACE/genesense-npm/\$WRAPPER_PACKAGE" > /dev/null 2>&1
          npx genesense --version
          echo "fn main\(\) \{ let y = 2; \}" > test2\.rs
          npx genesense test2\.rs --json > /dev/null
          echo "Genesense NPM wrapper OK"
''',
new_steps, text)

with open('.github/workflows/release.yml', 'w') as f:
    f.write(text)
