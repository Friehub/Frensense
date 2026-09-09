import re
content = open("frensense-engine/src/pattern/scorer.rs").read()

# Replace the raw_dim closure in score_against_corpus_with_evidence_impl
old_closure = r"""        let mut raw_dim = \|target: &FunctionFingerprint, is_negative: bool\| -> RawDimensions \{
            if let Some\(cache\) = dim_cache \{
                let key = fingerprint_id\(target\);
                if let Some\(cached\) = cache\.get\(&key\) \{
                    return \*cached;
                \}
            \}
            Self::raw_dimensions\(candidate, target, is_negative\)
        \};"""

new_closure = """        let mut raw_dim = |target: &FunctionFingerprint, _is_negative: bool| -> crate::pattern::similarity::RawDimensions {
            if let Some(cache) = dim_cache {
                let key = fingerprint_id(target);
                if let Some(cached) = cache.get(&key) {
                    return *cached;
                }
            }
            crate::pattern::similarity::compute_dimensions(candidate, target)
        };"""
content = re.sub(old_closure, new_closure, content)

# delete raw_dimensions function entirely
content = re.sub(r'pub\(crate\) fn raw_dimensions\(.*?\).*?RawDimensions \{.*?(?=^\})^\}', '', content, flags=re.DOTALL | re.MULTILINE)

open("frensense-engine/src/pattern/scorer.rs", "w").write(content)
