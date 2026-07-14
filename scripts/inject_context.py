import os
import re

CORPUS_DIR = "corpus/targets"

def get_env(path_str, content):
    path_str = path_str.lower()
    c = content.lower()
    
    if any(x in path_str for x in ["test", "spec", "cypress", "__tests__"]) or "describe(" in c or " it(" in c or "\nit(" in c:
        return "Test"
    if any(x in path_str for x in ["mock", "stub"]):
        return "Mock"
    if any(x in path_str for x in ["config", "settings"]):
        return "Config"
    if any(x in path_str for x in ["route", "controller"]) or "req, res" in c or "req: request" in c or "res.send" in c or "router." in c:
        return "RouteHandler"
    if any(x in path_str for x in ["lib", "util", "helper"]):
        return "Utility"
    return "Unknown"

def get_sens(content):
    c = content.lower()
    if any(x in c for x in ["password", "secret", "jwt", "token", "creditcard"]):
        return "High"
    if any(x in c for x in ["email", "user", "profile"]):
        return "Medium"
    if any(x in c for x in ["version", "metric", "telemetry"]):
        return "Low"
    return "Unknown"

for f in os.listdir(CORPUS_DIR):
    if f.endswith("_positive.ts") or f.endswith("_positive.js") or f.endswith("_positive.rs") or f.endswith("_positive.tsx"):
        pattern_id = f.split("_positive")[0]
        toml_path = os.path.join(CORPUS_DIR, f"{pattern_id}.toml")
        
        with open(os.path.join(CORPUS_DIR, f), "r") as pos_file:
            content = pos_file.read()
            
        env = get_env(f, content)
        sens = get_sens(content)
        
        toml_content = ""
        if os.path.exists(toml_path):
            with open(toml_path, "r") as tf:
                toml_content = tf.read()
            
        if "[expected_context]" in toml_content:
            toml_content = re.sub(r'\[expected_context\].*?(?=\n\[|$)', f'[expected_context]\nenvironment = "{env}"\nsensitivity = "{sens}"\nframeworks = []\n\n', toml_content, flags=re.DOTALL)
        else:
            toml_content += f'\n[expected_context]\nenvironment = "{env}"\nsensitivity = "{sens}"\nframeworks = []\n\n'
            
        with open(toml_path, "w") as tf:
            tf.write(toml_content)
                
print("Done")
