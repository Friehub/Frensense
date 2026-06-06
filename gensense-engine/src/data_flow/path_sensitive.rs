// SPDX-License-Identifier: MIT

use std::collections::HashSet;

use crate::cfg::{ControlFlowGraph, def_use::DefUseChain};

#[derive(Debug, Clone)]
pub struct BlockTaint {
    pub ins: HashSet<String>,
    pub outs: HashSet<String>,
    pub r#gen: HashSet<String>,
    pub kill: HashSet<String>,
}

#[derive(Debug, Clone)]
pub struct PathSensitiveTaint {
    pub blocks: Vec<BlockTaint>,
    source_patterns: Vec<String>,
    sink_patterns: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SinkHit {
    pub block_id: usize,
    pub sink_name: String,
    pub tainted_args: Vec<String>,
}

fn matches_pattern(name: &str, patterns: &[String]) -> bool {
    patterns
        .iter()
        .any(|p| name.contains(p.as_str()) || p.contains(name))
}

impl PathSensitiveTaint {
    pub fn new(source_patterns: Vec<String>, sink_patterns: Vec<String>) -> Self {
        Self {
            blocks: Vec::new(),
            source_patterns,
            sink_patterns,
        }
    }

    pub fn analyze(&mut self, cfg: &ControlFlowGraph, def_use: &DefUseChain) {
        let n = cfg.block_count();
        self.blocks = vec![
            BlockTaint {
                ins: HashSet::new(),
                outs: HashSet::new(),
                r#gen: HashSet::new(),
                kill: HashSet::new(),
            };
            n
        ];

        self.build_gen_kill(cfg, def_use);
        self.propagate(cfg);
    }

    fn build_gen_kill(&mut self, cfg: &ControlFlowGraph, def_use: &DefUseChain) {
        let source_patterns = self.source_patterns.clone();

        for (block_id, block) in self.blocks.iter_mut().enumerate() {
            let defs: Vec<_> = def_use
                .definitions
                .iter()
                .filter(|d| d.block_id == block_id)
                .collect();

            for def in &defs {
                block.kill.insert(def.name.clone());
            }

            let uses: Vec<_> = def_use
                .uses
                .iter()
                .filter(|u| u.block_id == block_id)
                .collect();

            for u in &uses {
                if matches_pattern(&u.name, &source_patterns) {
                    block.r#gen.insert(u.name.clone());
                }
            }

            for def in &defs {
                if matches_pattern(&def.name, &source_patterns) {
                    block.r#gen.insert(def.name.clone());
                }
            }

            let _ = cfg.block(block_id);
        }
    }

    fn propagate(&mut self, cfg: &ControlFlowGraph) {
        let n = self.blocks.len();
        let mut changed = true;
        let mut iteration = 0;
        let max_iterations = n * 2 + 10;

        while changed && iteration < max_iterations {
            changed = false;
            iteration += 1;

            for block_id in 0..n {
                let preds = cfg.predecessors(block_id);

                let new_in: HashSet<String> = if preds.is_empty() {
                    HashSet::new()
                } else {
                    let mut merged = HashSet::new();
                    for pred in &preds {
                        if let Some(pred_taint) = self.blocks.get(*pred) {
                            merged.extend(pred_taint.outs.iter().cloned());
                        }
                    }
                    merged
                };

                if new_in != self.blocks[block_id].ins {
                    self.blocks[block_id].ins.clone_from(&new_in);
                    changed = true;
                }

                let kill_set = self.blocks[block_id].kill.clone();
                let gen_set = self.blocks[block_id].r#gen.clone();

                let mut new_out = new_in;
                new_out.retain(|v| !kill_set.contains(v));
                new_out.extend(gen_set.iter().cloned());

                if new_out != self.blocks[block_id].outs {
                    self.blocks[block_id].outs = new_out;
                    changed = true;
                }
            }
        }
    }

    pub fn block_taint(&self, block_id: usize) -> Option<&BlockTaint> {
        self.blocks.get(block_id)
    }

    pub fn outs_for_block(&self, block_id: usize) -> HashSet<String> {
        self.blocks
            .get(block_id)
            .map(|b| b.outs.clone())
            .unwrap_or_default()
    }

    pub fn ins_for_block(&self, block_id: usize) -> HashSet<String> {
        self.blocks
            .get(block_id)
            .map(|b| b.ins.clone())
            .unwrap_or_default()
    }

    pub fn taint_at(&self, block_id: usize, var: &str) -> bool {
        self.blocks
            .get(block_id)
            .is_some_and(|b| b.outs.contains(var) || b.ins.contains(var))
    }

    pub fn sinks_reached(&self, cfg: &ControlFlowGraph, def_use: &DefUseChain) -> Vec<SinkHit> {
        let mut hits = Vec::new();

        for u in &def_use.uses {
            if !matches_pattern(&u.name, &self.sink_patterns) {
                continue;
            }

            let mut tainted_args = Vec::new();
            for arg_use in &def_use.uses {
                if arg_use.block_id == u.block_id
                    && arg_use.name != u.name
                    && is_variable(&arg_use.name)
                    && self.taint_at(arg_use.block_id, &arg_use.name)
                {
                    tainted_args.push(arg_use.name.clone());
                }
            }

            if !tainted_args.is_empty() && cfg.is_reachable(cfg.entry(), u.block_id) {
                hits.push(SinkHit {
                    block_id: u.block_id,
                    sink_name: u.name.clone(),
                    tainted_args,
                });
            }
        }

        hits
    }
}

fn is_variable(name: &str) -> bool {
    !name.contains('(') && !name.contains('"') && !name.contains('\'')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cfg::{build_cfg, def_use::compute_def_use};

    #[test]
    fn test_linear_taint_flow() {
        let source = r#"
fn linear() {
    let x = get_password();
    store_in_db(x);
}
"#;
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();

        let cfg = build_cfg(root, source, "rs");
        let def_use = compute_def_use(&cfg, source);

        let mut pst = PathSensitiveTaint::new(
            vec!["password".to_string()],
            vec!["store".to_string(), "db".to_string()],
        );
        pst.analyze(&cfg, &def_use);

        let hits = pst.sinks_reached(&cfg, &def_use);
        assert!(!hits.is_empty(), "should detect sink with tainted arg");
    }

    #[test]
    fn test_kill_set_populated() {
        let source = r#"
fn reassign() {
    let x = get_password();
    x = "safe";
    store_in_db(x);
}
"#;
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();

        let cfg = build_cfg(root, source, "rs");
        let def_use = compute_def_use(&cfg, source);

        let mut pst = PathSensitiveTaint::new(
            vec!["password".to_string()],
            vec!["store".to_string(), "db".to_string()],
        );
        pst.analyze(&cfg, &def_use);

        let kill = &pst.blocks[0].kill;
        assert!(
            kill.contains("x"),
            "reassignment of x should place x in the kill set"
        );
    }

    #[test]
    fn test_branch_paths_diverge() {
        let source = r#"
fn branch() {
    let x = get_password();
    if is_admin() {
        store_secure(x);
    }
    log("done");
}
"#;
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();

        let cfg = build_cfg(root, source, "rs");
        let def_use = compute_def_use(&cfg, source);

        let mut pst = PathSensitiveTaint::new(
            vec!["password".to_string()],
            vec!["store".to_string(), "log".to_string()],
        );
        pst.analyze(&cfg, &def_use);

        let hits = pst.sinks_reached(&cfg, &def_use);
        let has_store = hits.iter().any(|h| h.sink_name.contains("store"));
        assert!(
            has_store,
            "should detect store_secure as reachable from tainted source"
        );
    }
}
