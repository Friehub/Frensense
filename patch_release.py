import re

with open('.github/workflows/release.yml', 'r') as f:
    text = f.read()

cargo_publish_steps = """      - name: Publish frensense-lang
        run: cargo publish -p frensense-lang --token ${{ secrets.CARGO_REGISTRY_TOKEN }}
      - name: Publish frensense-frc
        run: cargo publish -p frensense-frc --token ${{ secrets.CARGO_REGISTRY_TOKEN }}
      - name: Publish frensense-engine
        run: cargo publish -p frensense-engine --token ${{ secrets.CARGO_REGISTRY_TOKEN }}
      - name: Publish frensense-providers
        run: cargo publish -p frensense-providers --token ${{ secrets.CARGO_REGISTRY_TOKEN }}
      - name: Publish frensense-bundler
        run: cargo publish -p frensense-bundler --token ${{ secrets.CARGO_REGISTRY_TOKEN }}
      - name: Publish frensense
        run: cargo publish -p frensense --token ${{ secrets.CARGO_REGISTRY_TOKEN }} --no-default-features --features rust,typescript,fingerprinting
      - name: Publish genesense wrapper
        run: cargo publish -p genesense --token ${{ secrets.CARGO_REGISTRY_TOKEN }}"""

text = re.sub(
r'''      - name: Publish Engine
        run: \|
          cargo publish -p frensense-engine \\
            --token \$\{\{ secrets\.CARGO_REGISTRY_TOKEN \}\}
      - name: Publish CLI
        run: \|
          cargo publish -p frensense \\
            --token \$\{\{ secrets\.CARGO_REGISTRY_TOKEN \}\} \\
            --no-default-features \\
            --features rust,typescript,fingerprinting
      - name: Publish Genesense Wrapper
        run: \|
          cargo publish -p genesense \\
            --token \$\{\{ secrets\.CARGO_REGISTRY_TOKEN \}\}''',
cargo_publish_steps, text)

npm_test_steps = """      - name: Test Genesense NPM Wrapper before publish
        run: |
          MAIN_PACKAGE=$(npm pack --silent 2>&1 | tail -1)
          cd genesense-npm
          WRAPPER_PACKAGE=$(npm pack --silent 2>&1 | tail -1)
          TESTDIR=$(mktemp -d)
          cd "$TESTDIR"
          npm init -y > /dev/null
          npm install "$GITHUB_WORKSPACE/$MAIN_PACKAGE" > /dev/null 2>&1
          npm install "$GITHUB_WORKSPACE/genesense-npm/$WRAPPER_PACKAGE" > /dev/null 2>&1
          npx genesense --version
          echo "fn main() { let y = 2; }" > test2.rs
          npx genesense test2.rs --json > /dev/null
          echo "Genesense NPM wrapper OK"
"""

text = re.sub(
r'''      - name: Test Genesense NPM Wrapper before publish
        run: \|
          cd genesense-npm
          PACKAGE=\$\(npm pack --silent 2>&1 \| tail -1\)
          echo "Genesense wrapper package: \$PACKAGE"
          TESTDIR=\$\(mktemp -d\)
          cd "\$TESTDIR"
          npm init -y > /dev/null
          npm install "\$GITHUB_WORKSPACE/genesense-npm/\$PACKAGE" > /dev/null 2>&1
          npx genesense --version
          echo "fn main\(\) \{ let y = 2; \}" > test2\.rs
          npx genesense test2\.rs --json > /dev/null
          echo "Genesense NPM wrapper OK"
''',
npm_test_steps, text)

with open('.github/workflows/release.yml', 'w') as f:
    f.write(text)
