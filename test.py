import sys
import collections
import re
from typing import Dict, List, Tuple

def analyze_log(filename):
    with open(filename) as f:
        content = f.read()
    
    # Just print the summary to see the duplicate counts!
    print("Run script")
