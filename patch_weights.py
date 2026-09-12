import os

new_array = "    0.08, 0.11, 0.06, 0.02, 0.02, 0.16, 0.07, 0.12, 0.09, 0.06, 0.07, 0.03, 0.03, 0.04, 0.04,\n"

files = [
    "frensense-engine/src/pattern/weight_learner.rs",
    "frensense-bundler/src/pattern/weight_learner.rs"
]

for file in files:
    if os.path.exists(file):
        with open(file, "r") as f:
            lines = f.readlines()
        
        with open(file, "w") as f:
            for i, line in enumerate(lines):
                if line.strip().startswith("0.08, 0.11,"):
                    f.write(new_array)
                else:
                    f.write(line)
