import re

with open("src/engine/project/runner.rs", "r") as f:
    code = f.read()

old_arr = """    let default_sqli_weights: [f64; 15] = [
        0.05, 0.10, 0.04, 0.02, 0.02, 0.05, 0.05, 0.04, 0.20, 0.25, 0.03, 0.05, 0.03, 0.02, 0.03,
        0.03,
    ];"""

new_arr = """    let default_sqli_weights: [f64; 15] = [
        0.05, 0.10, 0.04, 0.02, 0.02, 0.05, 0.05, 0.04, 0.20, 0.25, 0.03, 0.05, 0.03, 0.02, 0.03,
    ];"""

code = code.replace(old_arr, new_arr)

with open("src/engine/project/runner.rs", "w") as f:
    f.write(code)

