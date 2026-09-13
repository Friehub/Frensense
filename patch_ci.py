import re

with open('.github/workflows/ci.yml', 'r') as f:
    text = f.read()

# Replace manual cache in bench job
text = re.sub(
r'''      - name: Cache Cargo dependencies
        uses: actions/cache@v4
        with:
          path: \|
            ~/\.cargo/bin/
            ~/\.cargo/registry/index/
            ~/\.cargo/registry/cache/
            ~/\.cargo/git/cache/
            target/
          key: \$\{\{ runner\.os \}\}-cargo-bench-\$\{\{ hashFiles\('\*\*/Cargo\.lock'\) \}\}''',
r'''      - name: Setup Rust Cache
        uses: Swatinem/rust-cache@v2''',
text)

# Replace manual cache in benchmark-apps job
text = re.sub(
r'''      - name: Cache Cargo dependencies
        uses: actions/cache@v4
        with:
          path: \|
            ~/\.cargo/bin/
            ~/\.cargo/registry/index/
            ~/\.cargo/registry/cache/
            ~/\.cargo/git/cache/
            target/
          key: \$\{\{ runner\.os \}\}-cargo-apps-\$\{\{ hashFiles\('\*\*/Cargo\.lock'\) \}\}''',
r'''      - name: Setup Rust Cache
        uses: Swatinem/rust-cache@v2''',
text)

with open('.github/workflows/ci.yml', 'w') as f:
    f.write(text)
