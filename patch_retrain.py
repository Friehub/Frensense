import re
with open("src/bin/retrain-calibration.rs", "r") as f:
    code = f.read()

old_w = """        let default_w = &[
            0.10, 0.20, 0.08, 0.04, 0.03, 0.10, 0.08, 0.08, 0.06, 0.12, 0.06, 0.10, 0.03, 0.02,
            0.04, 0.04,
        ];"""
new_w = """        let default_w = &[
            0.10, 0.20, 0.08, 0.04, 0.03, 0.18, 0.08, 0.06, 0.12, 0.06, 0.10, 0.03, 0.02,
            0.04, 0.04,
        ];"""

code = code.replace(old_w, new_w)

with open("src/bin/retrain-calibration.rs", "w") as f:
    f.write(code)
