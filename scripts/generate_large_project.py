# scripts/generate_large_project.py
import os

def generate_project(root, num_files, funcs_per_file):
    os.makedirs(root, exist_ok=True)
    for i in range(num_files):
        path = os.path.join(root, f"file_{i}.rs")
        with open(path, "w") as f:
            for j in range(funcs_per_file):
                f.write(f"fn func_{i}_{j}() {{\n")
                f.write(f"    let x = {j};\n")
                f.write(f"    println!(\"{{}}\", x);\n")
                f.write("}\n\n")

if __name__ == "__main__":
    generate_project("bench_samples/large_project", 100, 50)
