import os

files = [
    "frensense-bundler/src/builder.rs",
    "frensense-bundler/src/pattern/weight_learner.rs",
    "src/engine/project/runner.rs",
    "frensense-engine/src/corpus/bundle.rs",
    "frensense-engine/src/corpus/registry.rs",
    "frensense-engine/src/pattern/scorer.rs",
    "frensense-engine/src/pattern/weight_learner.rs"
]

for file in files:
    if os.path.exists(file):
        with open(file, "r") as f:
            code = f.read()
        code = code.replace("[f64; 16]", "[f64; 15]")
        # also check for hardcoded arrays of 16 elements if any
        # weights = [x, y, z, ... (16 items)]
        # this might be harder to regex, let's just do [f64; 16] first
        with open(file, "w") as f:
            f.write(code)

