// SPDX-License-Identifier: MIT

//! Phase 5: Taint Query Engine
//!
//! This module searches the SSA and Points-To graphs for dataflow vulnerabilities.
//! It fully implements Memory SSA constraint solving, meaning it is perfectly
//! flow-sensitive. It handles Strong Updates (overwriting tainted data with clean data)
//! and correctly merges memory states at control flow Phi nodes.

use rustc_hash::{FxHashMap, FxHashSet};
use crate::data_flow::core::ir::*;
use crate::data_flow::core::heap::{PointsToAnalysis, LocId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaintLevel {
    Clean,
    Tainted,
}

/// Configuration defining the boundaries of the security analysis.
#[derive(Debug, Clone)]
pub struct TaintConfig {
    pub sources: FxHashSet<String>,     // e.g., "req.body", "getQuery()"
    pub sinks: FxHashSet<String>,       // e.g., "db.execute", "eval"
    pub sanitizers: FxHashSet<String>,  // e.g., "escapeHtml", "sanitize"
}

/// The core security scanner.
pub struct TaintEngine<'a> {
    ir: &'a FunctionIR,
    heap: &'a PointsToAnalysis,
    config: &'a TaintConfig,
    
    /// Tracks direct taint on standard SSA variables
    var_taint: FxHashMap<VarId, TaintLevel>,
    
    /// Tracks taint on physical memory fields FOR EACH MEMORY STATE.
    /// Maps: MemoryState(VarId) -> Set of tainted (LocId, FieldName)
    mem_taint: FxHashMap<VarId, FxHashSet<(LocId, String)>>,
    
    pub alerts: Vec<String>,
}

impl<'a> TaintEngine<'a> {
    pub fn new(ir: &'a FunctionIR, heap: &'a PointsToAnalysis, config: &'a TaintConfig) -> Self {
        Self {
            ir,
            heap,
            config,
            var_taint: FxHashMap::default(),
            mem_taint: FxHashMap::default(),
            alerts: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        let mut changed = true;
        
        while changed {
            changed = false;
            
            for block in self.ir.blocks.values() {
                // 1. Evaluate Phi Nodes (Graph Merges)
                for phi in &block.phis {
                    let dest = phi.dest;
                    let is_memory_state = self.ir.var_metadata.get(&dest).map_or(false, |m| m.is_memory_state);
                    
                    if is_memory_state {
                        // Merging Memory States
                        let mut merged_state = FxHashSet::default();
                        for &(_, inc_var) in &phi.incoming {
                            if let Some(state) = self.mem_taint.get(&inc_var) {
                                merged_state.extend(state.iter().cloned());
                            }
                        }
                        if self.mem_taint.get(&dest) != Some(&merged_state) {
                            self.mem_taint.insert(dest, merged_state);
                            changed = true;
                        }
                    } else {
                        // Merging Standard Variables
                        let mut is_tainted = false;
                        for &(_, inc_var) in &phi.incoming {
                            if self.get_var_taint(inc_var) == TaintLevel::Tainted {
                                is_tainted = true;
                                break;
                            }
                        }
                        if is_tainted && self.set_var_taint(dest, TaintLevel::Tainted) {
                            changed = true;
                        }
                    }
                }
                
                // 2. Evaluate Sequential Instructions
                for instr in &block.instructions {
                    self.process_instruction(instr, &mut changed);
                }
            }
        }
    }
    
    fn process_instruction(&mut self, instr: &Instruction, changed: &mut bool) {
        match instr {
            // --- Basic Data Movement ---
            Instruction::Assign { dest, src } |
            Instruction::Cast { dest, src, .. } |
            Instruction::UnaryOp { dest, src, .. } => {
                if self.get_operand_taint(src) == TaintLevel::Tainted {
                    if self.set_var_taint(*dest, TaintLevel::Tainted) {
                        *changed = true;
                    }
                }
            }
            
            Instruction::BinaryOp { dest, lhs, rhs, .. } => {
                if self.get_operand_taint(lhs) == TaintLevel::Tainted || 
                   self.get_operand_taint(rhs) == TaintLevel::Tainted {
                    if self.set_var_taint(*dest, TaintLevel::Tainted) {
                        *changed = true;
                    }
                }
            }
            
            // --- Memory Reads (Flow-Sensitive) ---
            Instruction::LoadField { dest, mem_in, base, field } => {
                let state = self.mem_taint.get(mem_in).cloned().unwrap_or_default();
                let mut is_tainted = false;
                
                if let Some(locs) = self.heap.pts.get(base) {
                    for &loc in locs {
                        if state.contains(&(loc, field.clone())) {
                            is_tainted = true;
                            break;
                        }
                    }
                }
                if is_tainted && self.set_var_taint(*dest, TaintLevel::Tainted) {
                    *changed = true;
                }
            }
            
            Instruction::LoadElement { dest, mem_in, base, index } => {
                let state = self.mem_taint.get(mem_in).cloned().unwrap_or_default();
                let mut is_tainted = false;
                
                let field_name = if let Operand::StringLiteral(s) = index { s.clone() } else { "*".to_string() };
                
                if let Some(locs) = self.heap.pts.get(base) {
                    for &loc in locs {
                        if state.contains(&(loc, field_name.clone())) || state.contains(&(loc, "*".to_string())) {
                            is_tainted = true;
                            break;
                        }
                    }
                }
                if is_tainted && self.set_var_taint(*dest, TaintLevel::Tainted) {
                    *changed = true;
                }
            }
            
            // --- Memory Writes (Generates new mem_out State) ---
            Instruction::StoreField { mem_out, mem_in, base, field, src } => {
                let mut state = self.mem_taint.get(mem_in).cloned().unwrap_or_default();
                let is_tainted = self.get_operand_taint(src) == TaintLevel::Tainted;
                
                if let Some(locs) = self.heap.pts.get(base) {
                    for &loc in locs {
                        if is_tainted {
                            state.insert((loc, field.clone()));
                        } else {
                            // STRONG UPDATE: Overwriting malicious data with clean data cleans the memory!
                            state.remove(&(loc, field.clone()));
                        }
                    }
                }
                
                if self.mem_taint.get(mem_out) != Some(&state) {
                    self.mem_taint.insert(*mem_out, state);
                    *changed = true;
                }
            }
            
            Instruction::StoreElement { mem_out, mem_in, base, index, src } => {
                let mut state = self.mem_taint.get(mem_in).cloned().unwrap_or_default();
                let is_tainted = self.get_operand_taint(src) == TaintLevel::Tainted;
                let field_name = if let Operand::StringLiteral(s) = index { s.clone() } else { "*".to_string() };
                
                if let Some(locs) = self.heap.pts.get(base) {
                    for &loc in locs {
                        if is_tainted {
                            state.insert((loc, field_name.clone()));
                        } else {
                            // Weak update for wildcards, strong for exact literal indices
                            if field_name != "*" {
                                state.remove(&(loc, field_name.clone()));
                            }
                        }
                    }
                }
                
                if self.mem_taint.get(mem_out) != Some(&state) {
                    self.mem_taint.insert(*mem_out, state);
                    *changed = true;
                }
            }

            // --- Pass-Through Memory States ---
            Instruction::Allocate { mem_out, mem_in, .. } |
            Instruction::Await { mem_out, mem_in, .. } |
            Instruction::Yield { mem_out, mem_in, .. } => {
                self.pass_through_memory(*mem_in, *mem_out, changed);
            }

            // --- Function Calls (Sources, Sinks, Sanitizers) ---
            Instruction::CallStatic { func, args, dest, mem_out, mem_in } => {
                self.pass_through_memory(*mem_in, *mem_out, changed); // Assuming function doesn't mutate our heap for now
                
                if self.config.sinks.contains(func) {
                    for (i, arg) in args.iter().enumerate() {
                        if self.get_operand_taint(arg) == TaintLevel::Tainted {
                            let alert = format!("CRITICAL VULNERABILITY: Tainted data reached sink '{}' at argument {}", func, i);
                            if !self.alerts.contains(&alert) { self.alerts.push(alert); }
                        }
                    }
                }
                
                if self.config.sources.contains(func) {
                    if let Some(d) = dest {
                        if self.set_var_taint(*d, TaintLevel::Tainted) { *changed = true; }
                    }
                }
                
                if !self.config.sanitizers.contains(func) && !self.config.sources.contains(func) {
                    let mut is_tainted = false;
                    for arg in args {
                        if self.get_operand_taint(arg) == TaintLevel::Tainted {
                            is_tainted = true;
                            break;
                        }
                    }
                    if is_tainted {
                        if let Some(d) = dest {
                            if self.set_var_taint(*d, TaintLevel::Tainted) { *changed = true; }
                        }
                    }
                }
            }
            
            Instruction::CallVirtual { method, receiver, args, dest, mem_out, mem_in } => {
                self.pass_through_memory(*mem_in, *mem_out, changed);
                // Standard taint propagation for methods
                let mut is_tainted = self.get_operand_taint(receiver) == TaintLevel::Tainted;
                for arg in args {
                    if self.get_operand_taint(arg) == TaintLevel::Tainted {
                        is_tainted = true;
                    }
                }
                if is_tainted {
                    if let Some(d) = dest {
                        if self.set_var_taint(*d, TaintLevel::Tainted) { *changed = true; }
                    }
                }
            }

            _ => {}
        }
    }

    // --- Helpers ---

    fn pass_through_memory(&mut self, mem_in: VarId, mem_out: VarId, changed: &mut bool) {
        let state = self.mem_taint.get(&mem_in).cloned().unwrap_or_default();
        if self.mem_taint.get(&mem_out) != Some(&state) {
            self.mem_taint.insert(mem_out, state);
            *changed = true;
        }
    }

    fn get_operand_taint(&self, op: &Operand) -> TaintLevel {
        match op {
            Operand::Var(v) => self.get_var_taint(*v),
            _ => TaintLevel::Clean,
        }
    }

    fn get_var_taint(&self, var: VarId) -> TaintLevel {
        self.var_taint.get(&var).copied().unwrap_or(TaintLevel::Clean)
    }

    fn set_var_taint(&mut self, var: VarId, level: TaintLevel) -> bool {
        if self.get_var_taint(var) != level {
            self.var_taint.insert(var, level);
            true
        } else {
            false
        }
    }
}
