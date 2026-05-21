// SPDX-License-Identifier: MIT
// GenSense Unified Static Analysis & Information Systems Simulation Engine
//
// Combines:
// 1. Compressed Sparse Row (CSR) Directed Hermitian Laplacian.
// 2. High-speed O(M) Power Iteration with Deflation for Fiedler Vector spectral audits.
// 3. Normalized N-gram token fingerprinting & Jaccard similarity clone detection.
// 4. Shannon Entropy secrets scanning for high-entropy hardcoded keys.
// 5. Inter-Procedural Taint-Flow verification for sensitive database storage.
// 6. Mutual Information (MI) risk assessment between AST patterns and bugs.
// 7. AST Syntactic Hamming Distance for ultra-fast incremental diff checking.
// 8. Huffman Coding tree for schema-aware AST token compression.
//
// Complies with all strict Rust compiler checks.

use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::time::Instant;

/// ============================================================================
/// COMPONENT 1: COMPLEX NUMBERS IMPLEMENTATION (For Hermitian Algebra)
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    pub const fn zero() -> Self {
        Self { re: 0.0, im: 0.0 }
    }

    pub const fn real(re: f64) -> Self {
        Self { re, im: 0.0 }
    }

    pub fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }

    pub fn sub(self, other: Self) -> Self {
        Self {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }

    pub fn mul(self, other: Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }

    pub fn scale(self, factor: f64) -> Self {
        Self {
            re: self.re * factor,
            im: self.im * factor,
        }
    }

    pub fn conj(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    pub fn norm_sq(self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    pub fn norm(self) -> f64 {
        self.norm_sq().sqrt()
    }
}

/// ============================================================================
/// COMPONENT 2: COMPRESSED SPARSE ROW (CSR) COMPLEX MATRIX REPRESENTATION
/// ============================================================================

#[derive(Debug, Clone)]
pub struct CsrMatrix {
    pub size: usize,
    pub values: Vec<Complex>,
    pub col_indices: Vec<usize>,
    pub row_offsets: Vec<usize>,
}

impl CsrMatrix {
    /// O(M) Sparse Matrix-Vector Multiplication (SpMV): y = A * x
    pub fn spmv(&self, x: &[Complex]) -> Vec<Complex> {
        let mut y = vec![Complex::zero(); self.size];

        for i in 0..self.size {
            let start = self.row_offsets[i];
            let end = self.row_offsets[i + 1];
            let mut sum = Complex::zero();

            for k in start..end {
                let val = self.values[k];
                let col = self.col_indices[k];
                sum = sum.add(val.mul(x[col]));
            }

            y[i] = sum;
        }

        y
    }
}

/// ============================================================================
/// PART 1: COMPRESSED DIRECTED HERMITIAN LAPLACIAN SYSTEM
/// ============================================================================

pub struct SparseDirectedGraph {
    pub size: usize,
    pub symbol_map: HashMap<String, usize>,
    pub edges: Vec<(usize, usize, f64)>,
}

impl SparseDirectedGraph {
    pub fn new(symbols: &[&str]) -> Self {
        let size = symbols.len();
        let mut symbol_map = HashMap::new();
        for (idx, &sym) in symbols.iter().enumerate() {
            symbol_map.insert(sym.to_string(), idx);
        }
        Self {
            size,
            symbol_map,
            edges: Vec::new(),
        }
    }

    pub fn add_directed_edge(&mut self, from: &str, to: &str, weight: f64) {
        if let (Some(&u), Some(&v)) = (self.symbol_map.get(from), self.symbol_map.get(to)) {
            self.edges.push((u, v, weight));
        }
    }

    /// Builds the Directed Normalized Hermitian Laplacian (L_H) directly in CSR format.
    pub fn build_hermitian_laplacian_csr(&self, theta: f64) -> CsrMatrix {
        let n = self.size;
        let mut degrees = vec![0.0; n];
        let mut adj_map: HashMap<(usize, usize), f64> = HashMap::new();

        for &(u, v, w) in &self.edges {
            degrees[u] += w;
            degrees[v] += w;
            adj_map.insert((u, v), w);
        }

        let mut values = Vec::new();
        let mut col_indices = Vec::new();
        let mut row_offsets = vec![0; n + 1];

        for i in 0..n {
            row_offsets[i] = values.len();

            for j in 0..n {
                if i == j {
                    if degrees[i] > 0.0 {
                        values.push(Complex::real(1.0));
                        col_indices.push(j);
                    }
                } else {
                    let w_ij = adj_map.get(&(i, j)).copied().unwrap_or(0.0);
                    let w_ji = adj_map.get(&(j, i)).copied().unwrap_or(0.0);
                    let sym_weight = w_ij + w_ji;

                    if sym_weight > 0.0 {
                        let deg_i = degrees[i];
                        let deg_j = degrees[j];

                        if deg_i > 0.0 && deg_j > 0.0 {
                            let phase = if w_ij > w_ji {
                                theta
                            } else if w_ji > w_ij {
                                -theta
                            } else {
                                0.0
                            };

                            let hermitian_entry = Complex::new(
                                sym_weight * phase.cos(),
                                sym_weight * phase.sin(),
                            );

                            let norm_factor = 1.0 / (deg_i * deg_j).sqrt();
                            let val = hermitian_entry.scale(norm_factor);
                            
                            values.push(val);
                            col_indices.push(j);
                        }
                    }
                }
            }
        }
        row_offsets[n] = values.len();

        CsrMatrix {
            size: n,
            values,
            col_indices,
            row_offsets,
        }
    }

    /// Computes the Fiedler vector using CSR-accelerated SpMV operations.
    pub fn compute_fiedler_vector_csr(&self, b_csr: &CsrMatrix) -> Vec<Complex> {
        let n = self.size;
        let val_const = 1.0 / (n as f64).sqrt();
        let v1 = vec![Complex::real(val_const); n];
        let mut x = vec![Complex::new(0.5, 0.5); n];

        for _ in 0..30 {
            let y = b_csr.spmv(&x);

            let mut dot = Complex::zero();
            for i in 0..n {
                dot = dot.add(y[i].mul(v1[i].conj()));
            }

            let mut y_def = vec![Complex::zero(); n];
            for i in 0..n {
                y_def[i] = y[i].sub(dot.mul(v1[i]));
            }

            let mut norm_sum = 0.0;
            for i in 0..n {
                norm_sum += y_def[i].norm_sq();
            }
            let norm = norm_sum.sqrt();

            if norm > 1e-9 {
                for i in 0..n {
                    x[i] = y_def[i].scale(1.0 / norm);
                }
            }
        }

        x
    }
}

/// ============================================================================
/// PART 2: ANONYMIZED N-GRAM FINGERPRINTING & JACCARD SIMILARITY DETECTOR
/// ============================================================================

#[derive(Debug, Clone)]
pub struct FunctionFingerprint {
    pub file_path: String,
    pub function_name: String,
    pub line: usize,
    pub ngram_hashes: HashSet<u64>,
}

pub struct FingerprintEngine;

impl FingerprintEngine {
    pub fn extract_fingerprints(
        file_path: &str,
        function_name: &str,
        line: usize,
        source_code: &str,
    ) -> FunctionFingerprint {
        let keywords: HashSet<&str> = [
            "let", "mut", "return", "println!", "fn", "if", "else", "for", "in", "match"
        ].iter().cloned().collect();

        let normalized_tokens: Vec<String> = source_code
            .split_whitespace()
            .filter(|t| !t.is_empty() && !t.starts_with("//"))
            .map(|t| {
                let clean = t.trim_matches(|c: char| !c.is_alphanumeric() && c != '!');
                if clean.is_empty() {
                    return t.to_string();
                }

                if keywords.contains(clean) {
                    clean.to_string()
                } else if clean.chars().all(|c| c.is_numeric()) {
                    "_LIT_".to_string()
                } else {
                    "_VAR_".to_string()
                }
            })
            .collect();

        let mut ngram_hashes = HashSet::new();

        if normalized_tokens.len() >= 5 {
            for i in 0..=(normalized_tokens.len().saturating_sub(5)) {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                normalized_tokens[i..i + 5].hash(&mut hasher);
                ngram_hashes.insert(hasher.finish());
            }
        }

        FunctionFingerprint {
            file_path: file_path.to_string(),
            function_name: function_name.to_string(),
            line,
            ngram_hashes,
        }
    }

    pub fn compute_jaccard_similarity(f1: &FunctionFingerprint, f2: &FunctionFingerprint) -> f64 {
        let intersection = f1.ngram_hashes.intersection(&f2.ngram_hashes).count();
        let union = f1.ngram_hashes.union(&f2.ngram_hashes).count();

        if union == 0 {
            return 0.0;
        }

        intersection as f64 / union as f64
    }
}

/// ============================================================================
/// PART 3: SHANNON ENTROPY SECRETS SCANNING ENGINE
/// ============================================================================

pub struct SecretsScanner;

impl SecretsScanner {
    pub fn calculate_shannon_entropy(value: &str) -> f64 {
        if value.is_empty() {
            return 0.0;
        }

        let mut char_counts = HashMap::new();
        for c in value.chars() {
            *char_counts.entry(c).or_insert(0.0) += 1.0;
        }

        let total_chars = value.chars().count() as f64;
        let mut entropy = 0.0;

        for &count in char_counts.values() {
            let p = count / total_chars;
            entropy -= p * p.log2();
        }

        entropy
    }

    pub fn scan_for_hardcoded_secrets(var_name: &str, value: &str) -> Option<&'static str> {
        let sensitive_patterns = [
            "private_key", "secret", "api_token", "apiKey", "password", "db_password"
        ];

        let name_lower = var_name.to_lowercase();
        let is_sensitive_name = sensitive_patterns.iter().any(|&pat| name_lower.contains(pat));

        if value.contains("-----BEGIN RSA PRIVATE KEY-----") || value.contains("-----BEGIN PRIVATE KEY-----") {
            return Some("CRITICAL: Hardcoded RSA/Cryptographic Private Key Header Detected!");
        }

        if is_sensitive_name {
            let entropy = Self::calculate_shannon_entropy(value);
            if entropy >= 4.5 && value.len() >= 16 {
                return Some("HIGH SEVERITY: Exposed High-Entropy Secret/API Key Detected!");
            }
        }

        None
    }
}

/// ============================================================================
/// PART 4: INTER-PROCEDURAL TAINT-FLOW ENGINE
/// ============================================================================

pub struct TaintFlowEngine;

impl TaintFlowEngine {
    pub fn verify_flow_safety(
        flow_path: &[&str],
        sanitizers: &[&str],
        sinks: &[&str],
    ) -> Result<(), &'static str> {
        let mut active_taint = true;

        for (idx, &node) in flow_path.iter().enumerate() {
            if idx == 0 {
                continue;
            }

            if sanitizers.contains(&node) {
                active_taint = false;
            }

            if sinks.contains(&node) && active_taint {
                return Err("CRITICAL SECURITY LEAK: Tainted PII data written to database without encryption!");
            }
        }

        Ok(())
    }
}

/// ============================================================================
/// PART 5: INFORMATION SYSTEMS - MUTUAL INFORMATION RISK ESTIMATION
/// ============================================================================

pub struct MutualInformationModel;

impl MutualInformationModel {
    /// Computes the Mutual Information (MI) between structural AST features and bug committing histories.
    /// MI(X; Y) = sum_x sum_y P(x, y) * log2( P(x, y) / (P(x) * P(y)) )
    pub fn compute_mutual_information(
        prob_feature: f64, // P(X = 1): probability of having structural vulnerability feature
        prob_bug: f64,     // P(Y = 1): probability of historical bug in code block
        prob_both: f64,    // P(X = 1, Y = 1): joint probability of both
    ) -> f64 {
        let px = [1.0 - prob_feature, prob_feature];
        let py = [1.0 - prob_bug, prob_bug];
        
        let mut pxy = vec![vec![0.0; 2]; 2];
        pxy[1][1] = prob_both;
        pxy[1][0] = prob_feature - prob_both;
        pxy[0][1] = prob_bug - prob_both;
        pxy[0][0] = 1.0 - (pxy[1][1] + pxy[1][0] + pxy[0][1]);

        let mut mutual_info = 0.0;

        for x in 0..2 {
            for y in 0..2 {
                let p_joint = pxy[x][y];
                if p_joint > 1e-9 {
                    let p_prod = px[x] * py[y];
                    if p_prod > 1e-9 {
                        mutual_info += p_joint * (p_joint / p_prod).log2();
                    }
                }
            }
        }

        mutual_info
    }
}

/// ============================================================================
/// PART 6: INFORMATION SYSTEMS - SYNTACTIC HAMMING DISTANCE ENGINE
/// ============================================================================

pub struct SyntacticHammingEngine;

impl SyntacticHammingEngine {
    /// Evaluates the Syntactic Hamming Distance of two token structural vectors.
    /// Useful for determining whether structure changed inside AST diff scans.
    pub fn calculate_structural_distance(ast1: &[&str], ast2: &[&str]) -> usize {
        let mut distance = 0;
        let min_len = ast1.len().min(ast2.len());

        for i in 0..min_len {
            if ast1[i] != ast2[i] {
                distance += 1;
            }
        }

        // Add difference in length
        distance += ast1.len().max(ast2.len()) - min_len;
        distance
    }
}

/// ============================================================================
/// PART 7: INFORMATION SYSTEMS - HUFFMAN COMPRESSION MODEL
/// ============================================================================

#[derive(Debug, Clone)]
pub enum HuffmanNode {
    Leaf { token: String, frequency: usize },
    Internal { left: Box<HuffmanNode>, right: Box<HuffmanNode> },
}

impl HuffmanNode {
    pub fn frequency(&self) -> usize {
        match self {
            Self::Leaf { frequency, .. } => *frequency,
            Self::Internal { left, right } => left.frequency() + right.frequency(),
        }
    }

    /// Recursively builds binary paths for tokens inside the schema-aware Huffman Tree.
    pub fn build_codes(&self, prefix: String, map: &mut HashMap<String, String>) {
        match self {
            Self::Leaf { token, .. } => {
                map.insert(token.clone(), prefix);
            }
            Self::Internal { left, right } => {
                left.build_codes(format!("{}0", prefix), map);
                right.build_codes(format!("{}1", prefix), map);
            }
        }
    }
}

pub struct HuffmanCompressor;

impl HuffmanCompressor {
    /// Simplifies the generation of a schema-aware Huffman tree for AST token sequences.
    pub fn build_ast_huffman_tree(frequencies: &[( &str, usize )]) -> HuffmanNode {
        let mut nodes: Vec<HuffmanNode> = frequencies
            .iter()
            .map(|&(tok, freq)| HuffmanNode::Leaf {
                token: tok.to_string(),
                frequency: freq,
            })
            .collect();

        while nodes.len() > 1 {
            // Sort ascending by frequency
            nodes.sort_by_key(|n| n.frequency());

            let left = Box::new(nodes.remove(0));
            let right = Box::new(nodes.remove(0));

            let parent = HuffmanNode::Internal { left, right };
            nodes.push(parent);
        }

        nodes.remove(0)
    }
}

/// ============================================================================
/// PART 8: UNIFIED HIGH-PERFORMANCE BENCHMARK DRIVER
/// ============================================================================
fn main() {
    println!("================================================================================");
    println!("GENSENSE ADVANCED UNIFIED MATH & INFORMATION SYSTEMS AUDIT SUITE");
    println!("================================================================================\n");

    // -------------------------------------------------------------------------
    // SYSTEM 1: N-GRAM & JACCARD SIMILARITY DETECTOR
    // -------------------------------------------------------------------------
    println!("[BENCHMARK 1] Sliding Window N-gram & Jaccard Similarity Audit");

    let code_block_1 = "
        let mut x = 10;
        let mut y = 20;
        let sum = x + y;
        println!(\"Sum is: {}\", sum);
        return sum;
    ";

    let code_block_2 = "
        let mut a = 10;
        let mut b = 20;
        let result = a + b;
        println!(\"Sum is: {}\", result);
        return result;
    ";

    let start_fingerprint = Instant::now();
    let fp1 = FingerprintEngine::extract_fingerprints("src/main.rs", "process_sum", 12, code_block_1);
    let fp2 = FingerprintEngine::extract_fingerprints("src/helper.rs", "evaluate_sum", 45, code_block_2);
    let elapsed_fingerprint = start_fingerprint.elapsed();

    let start_jaccard = Instant::now();
    let similarity = FingerprintEngine::compute_jaccard_similarity(&fp1, &fp2);
    let elapsed_jaccard = start_jaccard.elapsed();

    println!("  -> N-gram Extraction Speed:      {:?}", elapsed_fingerprint);
    println!("  -> Jaccard Similarity Speed:      {:?}", elapsed_jaccard);
    println!("  -> Measured Code Boilerplate Sim: {:.2}%", similarity * 100.0);

    if similarity >= 0.8 {
        println!("  [ALERT] Redundant Boilerplate clone flagged between '{}' and '{}'!", fp1.function_name, fp2.function_name);
    }

    println!("\n--------------------------------------------------------------------------------\n");

    // -------------------------------------------------------------------------
    // SYSTEM 2: SHANNON ENTROPY SECRETS AUDIT
    // -------------------------------------------------------------------------
    println!("[BENCHMARK 2] Shannon Entropy Secrets & API Key Audits");

    let normal_msg = "Your balance is updated successfully.";
    let rsa_key = "-----BEGIN RSA PRIVATE KEY-----\nMIIEowIBAAKCAQEA0Ue4sLgT5f...\n-----END RSA PRIVATE KEY-----";
    let stripe_api_secret = "stripe_test_key_placeholder_to_bypass_github_scanning_rules";

    let start_entropy = Instant::now();
    let ent_normal = SecretsScanner::calculate_shannon_entropy(normal_msg);
    let ent_rsa = SecretsScanner::calculate_shannon_entropy(rsa_key);
    let ent_stripe = SecretsScanner::calculate_shannon_entropy(stripe_api_secret);
    let elapsed_entropy = start_entropy.elapsed();

    println!("  -> Normal Text Entropy:      {:.4}", ent_normal);
    println!("  -> RSA Key Entropy:          {:.4}", ent_rsa);
    println!("  -> Stripe Key Entropy:       {:.4}", ent_stripe);

    if let Some(msg) = SecretsScanner::scan_for_hardcoded_secrets("stripe_secret_key", stripe_api_secret) {
        println!("  [ALERT] Variable 'stripe_secret_key': {}", msg);
    }

    if let Some(msg) = SecretsScanner::scan_for_hardcoded_secrets("jwt_token", rsa_key) {
        println!("  [ALERT] Variable 'jwt_token': {}", msg);
    }

    let dummy_check = SecretsScanner::scan_for_hardcoded_secrets("ui_label", normal_msg);
    println!("  -> Variable 'ui_label':      {}", if dummy_check.is_some() { "Flagged!" } else { "Passed (Safe)" });
    println!("  -> Secrets Scan Speed:       {:?}", elapsed_entropy);

    println!("\n--------------------------------------------------------------------------------\n");

    // -------------------------------------------------------------------------
    // SYSTEM 3: INTER-PROCEDURAL TAINT-FLOW DB WRITE AUDIT
    // -------------------------------------------------------------------------
    println!("[BENCHMARK 3] Inter-Procedural Taint-Flow Parameter Verification");

    let db_sinks = ["prisma.user.create", "postgres.query"];
    let sanitizers = ["bcrypt.hash", "crypto.encrypt"];

    let unsafe_flow_path = ["request.body.password", "user_controller", "prisma.user.create"];
    let safe_flow_path = ["request.body.password", "bcrypt.hash", "user_controller", "prisma.user.create"];

    let start_taint = Instant::now();
    let unsafe_result = TaintFlowEngine::verify_flow_safety(&unsafe_flow_path, &sanitizers, &db_sinks);
    let safe_result = TaintFlowEngine::verify_flow_safety(&safe_flow_path, &sanitizers, &db_sinks);
    let elapsed_taint = start_taint.elapsed();

    println!("  -> Unsafe Write Verification: {}", match unsafe_result {
        Ok(_) => "Passed (Unsafe!)",
        Err(e) => e,
    });
    println!("  -> Safe Write Verification:   {}", match safe_result {
        Ok(_) => "Passed (Safe)",
        Err(e) => e,
    });
    println!("  -> Taint Audit Speed:         {:?}", elapsed_taint);

    println!("\n--------------------------------------------------------------------------------\n");

    // -------------------------------------------------------------------------
    // SYSTEM 4: MUTUAL INFORMATION RISK ASSESSMENT
    // -------------------------------------------------------------------------
    println!("[BENCHMARK 4] Mutual Information Risk Modeling");

    let start_mi = Instant::now();
    let mi_score = MutualInformationModel::compute_mutual_information(0.12, 0.08, 0.075);
    let elapsed_mi = start_mi.elapsed();

    println!("  -> Mutual Information (Feature vs Bug): {:.6} bits", mi_score);
    println!("  -> Risk Inference Speed:                {:?}", elapsed_mi);

    println!("\n--------------------------------------------------------------------------------\n");

    // -------------------------------------------------------------------------
    // SYSTEM 5: SYNTACTIC HAMMING DIFF SCANNING
    // -------------------------------------------------------------------------
    println!("[BENCHMARK 5] Syntactic AST Hamming Edit Distance");

    let original_ast = ["let", "mut", "_VAR_", "=", "_LIT_", ";", "return", "_VAR_", ";"];
    let modified_ast = ["let", "mut", "_VAR_", "=", "_LIT_", ";", "_VAR_", "=", "_LIT_", ";", "return", "_VAR_", ";"];

    let start_hamming = Instant::now();
    let structural_dist = SyntacticHammingEngine::calculate_structural_distance(&original_ast, &modified_ast);
    let elapsed_hamming = start_hamming.elapsed();

    println!("  -> Structural Edit Hamming Distance: {} steps", structural_dist);
    println!("  -> AST Edit Distance Check Speed:    {:?}", elapsed_hamming);

    println!("\n--------------------------------------------------------------------------------\n");

    // -------------------------------------------------------------------------
    // SYSTEM 6: HUFFMAN COMPRESSION SPECIFICATION
    // -------------------------------------------------------------------------
    println!("[BENCHMARK 6] Schema-Aware AST Token Huffman Tree Compression");

    let token_freqs = [
        ("_VAR_", 6500),
        (";", 5800),
        ("let", 3200),
        ("=", 3100),
        ("_LIT_", 2800),
        ("mut", 1200),
        ("return", 800),
    ];

    let start_huffman = Instant::now();
    let tree = HuffmanCompressor::build_ast_huffman_tree(&token_freqs);
    let mut huffman_map = HashMap::new();
    tree.build_codes(String::new(), &mut huffman_map);
    let elapsed_huffman = start_huffman.elapsed();

    println!("  -> Huffman Tree Built.");
    for (tok, code) in &huffman_map {
        println!("     Token [{:<8}]: Huffman Code = {}", tok, code);
    }
    println!("  -> Compressed Representation Speed: {:?}", elapsed_huffman);

    println!("\n--------------------------------------------------------------------------------\n");

    // -------------------------------------------------------------------------
    // SYSTEM 7: DIRECTION-AWARE HERMITIAN CSR LAPLACIAN
    // -------------------------------------------------------------------------
    println!("[BENCHMARK 7] O(M) CSR Hermitian Laplacian Spectral Decomposition");

    let modules = ["Gateway", "Router", "AuthService", "BillingService", "Database"];
    let mut graph = SparseDirectedGraph::new(&modules);
    graph.add_directed_edge("Gateway", "Router", 1.0);
    graph.add_directed_edge("Router", "AuthService", 1.0);
    graph.add_directed_edge("AuthService", "BillingService", 1.0);
    graph.add_directed_edge("BillingService", "Database", 1.0);

    let theta_phase = std::f64::consts::FRAC_PI_3;

    let start_laplacian = Instant::now();
    let b_csr = graph.build_hermitian_laplacian_csr(theta_phase);
    let fiedler = graph.compute_fiedler_vector_csr(&b_csr);
    let elapsed_laplacian = start_laplacian.elapsed();

    println!("  -> CSR Herm Laplacian & Fiedler Solver Speed: {:?}", elapsed_laplacian);
    println!("  -> Total non-zero sparse matrix entries:       {}", b_csr.values.len());

    println!("\n  Fiedler vector coordinates resolved on sparse layout:");
    for (idx, val) in fiedler.iter().enumerate() {
        println!("     Node [{}]: {:.4} + {:.4}i", modules[idx], val.re, val.im);
    }
    println!("================================================================================");
}
