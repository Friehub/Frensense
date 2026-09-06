import subprocess
import os
import time

FRENSENSE_BIN = "./target/release/frensense"

def generate_large_ts(size_kb):
    path = f"bench_samples/memory_stress.ts"
    os.makedirs("bench_samples", exist_ok=True)
    with open(path, "w") as f:
        f.write("async function stress_test() {\n")
        f.write("    const start = \"password\";\n")
        for i in range(1, (size_kb * 10)): # Roughly 10 vars per KB
            f.write(f"    const x_{i} = x_{i-1 if i > 1 else 'start'};\n")
        f.write(f"    console.log(x_{(size_kb * 10) - 1});\n")
        f.write("}\n")
    return path

def profile_run(path):
    # Start the process
    start_time = time.time()
    proc = subprocess.Popen([FRENSENSE_BIN, path, "--json"], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    
    max_rss = 0
    while proc.poll() is None:
        try:
            # Get RSS in KB using ps
            output = subprocess.check_output(["ps", "-o", "rss=", "-p", str(proc.pid)])
            rss = int(output.strip())
            if rss > max_rss:
                max_rss = rss
        except:
            pass
        time.sleep(0.1)
    
    end_time = time.time()
    return max_rss, end_time - start_time

def main():
    print("# Frensense Semantic Memory Profile")
    print("| Size (KB) | Max RSS (MB) | Time (s) |")
    print("| :--- | :--- | :--- |")
    
    for size in [100, 500, 1000]: # 100KB, 500KB, 1MB of code
        path = generate_large_ts(size)
        rss, duration = profile_run(path)
        print(f"| {size} | {rss / 1024:.2f} | {duration:.2f} |")

if __name__ == "__main__":
    main()
