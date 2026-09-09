import re
content = open("frensense-engine/src/pattern/scorer.rs").read()

# I will find the block starting with "        0.0\n    } else {\n        intersection / union\n    }\n}\n\n/// Longest Common Subsequence (LCS) similarity metric."
# up to "lcs_len / max_len\n}\n" and delete it.
content = re.sub(r'/// Longest Common Subsequence \(LCS\) similarity metric\..*?lcs_len / max_len\n\}\n', '', content, flags=re.DOTALL)
open("frensense-engine/src/pattern/scorer.rs", "w").write(content)
