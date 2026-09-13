import re

with open('.github/workflows/release.yml', 'r') as f:
    text = f.read()

new_steps = """      - name: Test NPM package before publish
        run: |
          npm pack
          TESTDIR=$(mktemp -d)
          cd "$TESTDIR"
          npm init -y > /dev/null
          npm install "$GITHUB_WORKSPACE"/friehub-frensense-*.tgz
          npx frensense --version
          echo 'fn main() { let x = 1; }' > test.rs
          npx frensense test.rs --json > /dev/null
          echo 'NPM package OK'
"""

text = re.sub(
r'''      - name: Test NPM package before publish
        run: \|
          PACKAGE=\$\(npm pack --silent 2>&1 \| tail -1\)
          echo "Package: \$PACKAGE"
          TESTDIR=\$\(mktemp -d\)
          cd "\$TESTDIR"
          npm init -y > /dev/null
          npm install "\$GITHUB_WORKSPACE/\$PACKAGE" > /dev/null 2>&1
          npx frensense --version
          echo 'fn main\(\) \{ let x = 1; \}' > test\.rs
          npx frensense test\.rs --json > /dev/null
          echo 'NPM package OK'
''',
new_steps, text)

with open('.github/workflows/release.yml', 'w') as f:
    f.write(text)
