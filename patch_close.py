content = open("frensense-engine/src/pattern/scorer.rs").read()
content = content.replace("pub fn fingerprint_id", "}\n\npub fn fingerprint_id")
open("frensense-engine/src/pattern/scorer.rs", "w").write(content)
