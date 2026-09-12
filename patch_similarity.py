with open("frensense-engine/src/pattern/similarity.rs", "r") as f:
    code = f.read()

# 1. Remove from struct
code = code.replace("    pub semantic_api_sim: f64,\n", "")

# 2. Remove from weighted_score gate
code = code.replace("            .max(self.semantic_api_sim)\n", "")

# 3. Update w[16] to w[15] in weighted_score signature
code = code.replace("pub fn weighted_score(&self, w: &[f64; 16])", "pub fn weighted_score(&self, w: &[f64; 15])")

# 4. Update the vuln_score addition
old_vuln = """        let vuln_score = self.ngram_sim * w[0]
            + self.ast_sim * w[1]
            + self.signature_sim * w[2]
            + self.param_type_sim * w[3]
            + self.type_usage_sim * w[4]
            + self.semantic_sim * w[5]
            + self.semantic_api_sim * w[6]
            + self.cf_sim * w[7]
            + self.api_sim * w[8]
            + self.tainted_api_sim * w[9]
            + self.motif_sim * w[10]
            + self.flow_sim * w[11]
            + self.config_sim * w[12]
            + self.cf_order_sim * w[13]
            + self.arg_type_sim * w[14]
            + self.literal_concat_sim * w[15];"""

new_vuln = """        let vuln_score = self.ngram_sim * w[0]
            + self.ast_sim * w[1]
            + self.signature_sim * w[2]
            + self.param_type_sim * w[3]
            + self.type_usage_sim * w[4]
            + self.semantic_sim * w[5]
            + self.cf_sim * w[6]
            + self.api_sim * w[7]
            + self.tainted_api_sim * w[8]
            + self.motif_sim * w[9]
            + self.flow_sim * w[10]
            + self.config_sim * w[11]
            + self.cf_order_sim * w[12]
            + self.arg_type_sim * w[13]
            + self.literal_concat_sim * w[14];"""

code = code.replace(old_vuln, new_vuln)

# 5. as_array
code = code.replace("pub fn as_array(&self) -> [f64; 16]", "pub fn as_array(&self) -> [f64; 15]")
code = code.replace("            self.semantic_api_sim,\n", "")

# 6. Inside calculate()
old_calc = "    let semantic_sim = jaccard_sorted(&candidate.semantic_markers, &target.semantic_markers);"
new_calc = """    let semantic_sim = jaccard_sorted(&candidate.semantic_markers, &target.semantic_markers);
    let semantic_api_sim = if !candidate.semantic_api_calls.is_empty()
        && !target.semantic_api_calls.is_empty()
    {
        jaccard_sorted(
            &candidate.semantic_api_calls,
            &target.semantic_api_calls,
        )
    } else {
        0.0
    };
    let semantic_sim = semantic_sim.max(semantic_api_sim);"""

# I need to fix how semantic_api_sim is removed from the RawDimensions initialization!
old_init = """    RawDimensions {
        ngram_sim,
        ast_sim,
        signature_sim,
        param_type_sim,
        type_usage_sim,
        semantic_sim,
        semantic_api_sim,
        cf_sim,
        api_sim,
        tainted_api_sim,
        motif_sim,
        flow_sim,
        config_sim,
        cf_order_sim,
        arg_type_sim,
        literal_concat_sim,
    }"""
new_init = """    RawDimensions {
        ngram_sim,
        ast_sim,
        signature_sim,
        param_type_sim,
        type_usage_sim,
        semantic_sim,
        cf_sim,
        api_sim,
        tainted_api_sim,
        motif_sim,
        flow_sim,
        config_sim,
        cf_order_sim,
        arg_type_sim,
        literal_concat_sim,
    }"""

code = code.replace(old_init, new_init)

with open("frensense-engine/src/pattern/similarity.rs", "w") as f:
    f.write(code)

