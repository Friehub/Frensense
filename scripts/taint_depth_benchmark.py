# scripts/taint_depth_benchmark.py
import subprocess
import os
import json
import time

GENSENSE_BIN = "./target/release/gensense"

def generate_ts(depth):
    path = f"bench_samples/taint_{depth}.ts"
    with open(path, "w") as f:
        f.write("async function test_taint() {\n")
        # The source pattern is "password|secret|token|key"
        # We assign a value that matches the pattern to the first variable.
        f.write("    const start = \"password\";\n") 
        last_var = "start"
        for i in range(1, depth):
            curr_var = f"x_{i}"
            f.write(f"    const {curr_var} = {last_var};\n")
            last_var = curr_var
        # The sink pattern is "console\\.log"
        f.write(f"    console.log({last_var});\n")
        f.write("}\n")
    return path

def run_test(path):
    start = time.time()
    result = subprocess.run([GENSENSE_BIN, path, "--json"], capture_output=True, text=True)
    end = time.time()
    
    if result.returncode != 0:
        return False, end - start
    
    try:
        advisories = json.loads(result.stdout)
        # print(f"DEBUG: {path} -> {advisories}")
        # Search for TS_DATA_LEAK_TRACKER or any Advisory that mentions "Leak"
        found = any(a["rule_id"] == "TS_DATA_LEAK_TRACKER" or "Leak" in a["observation"] for a in advisories)
        return found, end - start
    except Exception as e:
        # print(f"ERROR: {e}")
        return False, end - start

def main():
    os.makedirs("bench_samples", exist_ok=True)
    depths = [1, 2, 5, 10, 20, 50] # Start smaller
    
    print("# GenSense Taint Depth Benchmark")
    print("| Depth | Detected | Time (s) |")
    print("| :--- | :--- | :--- |")
    
    for d in depths:
        path = generate_ts(d)
        detected, duration = run_test(path)
        status = "✅ YES" if detected else "❌ NO"
        print(f"| {d} | {status} | {duration:.2f} |")

if __name__ == "__main__":
    main()
