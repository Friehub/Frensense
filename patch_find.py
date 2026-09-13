with open("frensense-bundler/src/builder.rs", "r") as f:
    code = f.read()

import re

# We will replace the block where it does `find_corpus_file`.
# First, let's find that block.
# Wait, I'll just write a script to replace the function `find_corpus_file` and its calls.
