# scripts/generate_large_file.py
import os

def generate_large_rust(path, lines):
    with open(path, "w") as f:
        f.write("fn main() {\n")
        for i in range(lines):
            f.write(f"    let x_{i} = {i};\n")
            if i % 10 == 0:
                f.write(f"    println!(\"{{}}\", x_{i});\n")
        f.write("}\n")

if __name__ == "__main__":
    os.makedirs("bench_samples", exist_ok=True)
    generate_large_rust("bench_samples/large_file.rs", 10000)
