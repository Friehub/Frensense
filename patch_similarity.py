content = open("frensense-engine/src/pattern/similarity.rs").read()
if "pub fn compute_dimensions" not in content:
    compute_dim = """
pub fn compute_dimensions(
    candidate: &FunctionFingerprint,
    target: &FunctionFingerprint,
) -> RawDimensions {
    let ngram_sim = if candidate.weighted_ngram_hashes.is_empty()
        || target.weighted_ngram_hashes.is_empty()
    {
        jaccard(&candidate.ngram_hashes, &target.ngram_hashes)
    } else {
        crate::pattern::scorer::weighted_jaccard(
            &candidate.weighted_ngram_hashes,
            &target.weighted_ngram_hashes,
        )
    };

    let semantic_sim = jaccard(&candidate.semantic_markers, &target.semantic_markers);

    let ast_sim = if !candidate.skeleton_hashes.is_empty()
        && !target.skeleton_hashes.is_empty()
        && ngram_sim > 0.25
    {
        1.0 - crate::ast_distance::tree_edit_distance(
            &candidate.skeleton_hashes,
            &target.skeleton_hashes,
        )
    } else {
        jaccard(&candidate.structural_markers, &target.structural_markers)
    };

    let signature_sim = jaccard_sorted(&candidate.signature_ngrams, &target.signature_ngrams);
    let param_type_sim = jaccard_sorted(&candidate.param_type_ngrams, &target.param_type_ngrams);
    let type_usage_sim = type_usage_overlap(candidate, target);
    let cf_sim = jaccard(&candidate.control_flow_hashes, &target.control_flow_hashes);

    let api_sim_full = jaccard(&candidate.api_calls, &target.api_calls);
    let api_sim_seg = if !candidate.api_call_segments.is_empty() && !target.api_call_segments.is_empty() {
        jaccard(&candidate.api_call_segments, &target.api_call_segments)
    } else {
        0.0
    };
    let api_sim = api_sim_full.max(api_sim_seg);

    let motif_sim = containment(&candidate.motif_hashes, &target.motif_hashes);
    let flow_sim = containment(
        &candidate.data_flow_path_hashes,
        &target.data_flow_path_hashes,
    );

    // Mutual empty MUST be 1.0 so that weights don't zero out!
    let tainted_api_sim = if candidate.tainted_api_calls.is_empty() && target.tainted_api_calls.is_empty() {
        1.0
    } else if candidate.tainted_api_calls.is_empty() {
        0.0
    } else if target.tainted_api_calls.is_empty() {
        jaccard_sorted(&candidate.tainted_api_calls, &target.api_calls)
    } else {
        jaccard_sorted(&candidate.tainted_api_calls, &target.tainted_api_calls)
    };

    let config_sim = jaccard(
        &candidate.config_literal_hashes,
        &target.config_literal_hashes,
    );

    let cf_order_sim = if candidate.control_flow_sequence.is_empty()
        && target.control_flow_sequence.is_empty()
    {
        1.0
    } else {
        lcs_similarity(
            &candidate.control_flow_sequence,
            &target.control_flow_sequence,
        )
    };

    let arg_type_sim = if !candidate.argument_call_types.is_empty()
        && !target.argument_call_types.is_empty()
    {
        jaccard(&candidate.argument_call_types, &target.argument_call_types)
    } else {
        0.0
    };

    let literal_concat_sim = if !candidate.literal_pattern_hashes.is_empty()
        && !target.literal_pattern_hashes.is_empty()
    {
        jaccard(
            &candidate.literal_pattern_hashes,
            &target.literal_pattern_hashes,
        )
    } else {
        0.0
    };

    RawDimensions {
        ngram_sim,
        ast_sim,
        signature_sim,
        param_type_sim,
        type_usage_sim,
        semantic_sim,
        cf_sim,
        api_sim,
        motif_sim,
        flow_sim,
        tainted_api_sim,
        config_sim,
        cf_order_sim,
        arg_type_sim,
        literal_concat_sim,
    }
}
"""
    content += compute_dim
    open("frensense-engine/src/pattern/similarity.rs", "w").write(content)
