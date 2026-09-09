import re
content = open("frensense-engine/src/pattern/scorer.rs").read()
# Delete everything between `pub fn type_usage_overlap` and `#[cfg(test)]`
content = re.sub(r'pub fn type_usage_overlap.*?#\[cfg\(test\)\]', '#[cfg(test)]', content, flags=re.DOTALL)
open("frensense-engine/src/pattern/scorer.rs", "w").write(content)
