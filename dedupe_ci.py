with open('.github/workflows/ci.yml', 'r') as f:
    text = f.read()

text = text.replace(
'''      - name: Setup Rust Cache
        uses: Swatinem/rust-cache@v2

      - name: Setup Rust Cache
        uses: Swatinem/rust-cache@v2''',
'''      - name: Setup Rust Cache
        uses: Swatinem/rust-cache@v2''')

with open('.github/workflows/ci.yml', 'w') as f:
    f.write(text)
