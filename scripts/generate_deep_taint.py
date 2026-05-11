# scripts/generate_deep_taint.py
import os

def generate_deep_ts(path, depth):
    with open(path, "w") as f:
        f.write("async function test_taint() {\n")
        f.write("    const password = \"MY_SECRET_PASSWORD\";\n")
        for i in range(1, depth):
            f.write(f"    const x_{i} = x_{i-1 if i > 1 else 'password'};\n")
        f.write(f"    console.log(x_{depth-1});\n")
        f.write("}\n")

if __name__ == "__main__":
    os.makedirs("bench_samples", exist_ok=True)
    generate_deep_ts("bench_samples/deep_taint.ts", 100)
