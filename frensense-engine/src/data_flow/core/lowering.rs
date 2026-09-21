// SPDX-License-Identifier: MIT

//! Phase 1.5: AST Lowering Pass
//!
//! This module traverses the `tree-sitter` AST using `frensense-lang` and
//! builds the Unstructured FunctionIR.
//! 
//! NEW: It seamlessly integrates Memory SSA. The entire Heap is treated as a
//! single mutable variable `self.memory_var` during lowering. The SSA Builder
//! will automatically split this into `mem_1`, `mem_2` and generate Memory Phis!

use rustc_hash::FxHashMap;
use tree_sitter::Node;
use frensense_lang::{LanguageSpec, NodeRole};
use crate::data_flow::core::ir::*;

pub struct LoweringContext<'a> {
    pub spec: &'a dyn LanguageSpec,
    pub source: &'a str,
    pub ir: FunctionIR,
    
    /// The current BasicBlock being populated
    pub current_block: BlockId,
    
    /// Lexical Environment mapping variable names to their VarId
    pub env: Vec<FxHashMap<String, VarId>>,

    /// The single mutable VarId representing the entire Heap State.
    /// It is mutated by Store/Call instructions. The SSABuilder will automatically
    /// version this into mem_1, mem_2, mem_3 and generate Memory Phis.
    pub memory_var: VarId,
}

/// Helper to differentiate Assigning to a Variable vs Mutating an Object
pub enum LValue {
    Variable(VarId),
    Field { base: VarId, field: String },
    Element { base: VarId, index: Operand },
}

impl<'a> LoweringContext<'a> {
    pub fn new(spec: &'a dyn LanguageSpec, source: &'a str, func_name: String) -> Self {
        let ir = FunctionIR::new(func_name);
        let entry = ir.entry_block;
        let memory_var = ir.initial_memory_state; // Start with the initial parameter
        
        let mut ctx = Self {
            spec,
            source,
            ir,
            current_block: entry,
            env: vec![FxHashMap::default()], // Global/Function scope
            memory_var,
        };

        // If 'req', 'res' etc. are parameters, they would be added here in a real parser.
        ctx
    }

    pub fn resolve_identifier(&mut self, node: Node) -> Operand {
        let name = self.source[node.start_byte()..node.end_byte()].to_string();
        
        for scope in self.env.iter().rev() {
            if let Some(&var) = scope.get(&name) {
                return Operand::Var(var);
            }
        }
        
        let new_var = self.ir.new_var(VarMetadata {
            source_name: Some(name.clone()),
            type_name: None,
            byte_range: Some((node.start_byte(), node.end_byte())),
            is_memory_state: false,
        });
        
        self.env.last_mut().unwrap().insert(name, new_var);
        Operand::Var(new_var)
    }

    pub fn visit_lvalue(&mut self, node: Node) -> Option<LValue> {
        let role = self.spec.classify(node.kind());
        match role {
            NodeRole::Identifier => {
                if let Operand::Var(v) = self.resolve_identifier(node) {
                    Some(LValue::Variable(v))
                } else {
                    None
                }
            }
            NodeRole::MemberExpression { object_field, property_field } => {
                let obj_node = node.child_by_field_name(object_field)?;
                let prop_node = node.child_by_field_name(property_field)?;
                
                let base_op = self.visit_node(obj_node)?;
                let field_name = self.source[prop_node.start_byte()..prop_node.end_byte()].to_string();
                
                if let Operand::Var(base_var) = base_op {
                    Some(LValue::Field { base: base_var, field: field_name })
                } else {
                    None
                }
            }
            NodeRole::SubscriptExpression { object_field, index_field } => {
                let obj_node = node.child_by_field_name(object_field)?;
                let idx_node = node.child_by_field_name(index_field)?;
                
                let base_op = self.visit_node(obj_node)?;
                let index_op = self.visit_node(idx_node)?;
                
                if let Operand::Var(base_var) = base_op {
                    Some(LValue::Element { base: base_var, index: index_op })
                } else {
                    None
                }
            }
            _ => None
        }
    }

    pub fn visit_node(&mut self, node: Node) -> Option<Operand> {
        let role = self.spec.classify(node.kind());
        
        match role {
            // ─── MEMORY & VARIABLES ────────────────────────────────────────────────
            NodeRole::VariableDeclaration { declarator_field } => {
                let decl = node.child_by_field_name(declarator_field)?;
                self.visit_node(decl);
                None
            }
            NodeRole::AssignmentExpression { left_field, right_field } => {
                let left_node = node.child_by_field_name(left_field)?;
                let right_node = node.child_by_field_name(right_field)?;
                
                let rhs_op = self.visit_node(right_node).unwrap_or(Operand::Unknown);
                let lval = self.visit_lvalue(left_node);
                
                match lval {
                    Some(LValue::Variable(dest)) => {
                        self.ir.push_instruction(self.current_block, Instruction::Assign {
                            dest,
                            src: rhs_op,
                        });
                    }
                    Some(LValue::Field { base, field }) => {
                        // Memory Mutation! Consumes self.memory_var, and re-assigns it.
                        self.ir.push_instruction(self.current_block, Instruction::StoreField {
                            mem_out: self.memory_var, 
                            mem_in: self.memory_var,
                            base,
                            field,
                            src: rhs_op,
                        });
                    }
                    Some(LValue::Element { base, index }) => {
                        self.ir.push_instruction(self.current_block, Instruction::StoreElement {
                            mem_out: self.memory_var,
                            mem_in: self.memory_var,
                            base,
                            index,
                            src: rhs_op,
                        });
                    }
                    None => {}
                }
                None
            }
            NodeRole::MemberExpression { object_field, property_field } => {
                let obj_node = node.child_by_field_name(object_field)?;
                let prop_node = node.child_by_field_name(property_field)?;
                
                let base_op = self.visit_node(obj_node)?;
                let field_name = self.source[prop_node.start_byte()..prop_node.end_byte()].to_string();
                
                if let Operand::Var(base_var) = base_op {
                    let dest = self.ir.new_var(VarMetadata { source_name: None, type_name: None, byte_range: None, is_memory_state: false });
                    // Reads use the current memory state
                    self.ir.push_instruction(self.current_block, Instruction::LoadField {
                        dest,
                        mem_in: self.memory_var,
                        base: base_var,
                        field: field_name,
                    });
                    Some(Operand::Var(dest))
                } else {
                    None
                }
            }
            
            // ─── CALLS (Static, Virtual, Pointer) ─────────────────────────────────────
            NodeRole::Call { callee_field, args_field } => {
                let callee_node = node.child_by_field_name(callee_field)?;
                let args_node = node.child_by_field_name(args_field)?;
                
                let mut args = Vec::new();
                let mut cursor = args_node.walk();
                for child in args_node.children(&mut cursor) {
                    if child.is_named() {
                        if let Some(op) = self.visit_node(child) {
                            args.push(op);
                        }
                    }
                }
                
                let dest = self.ir.new_var(VarMetadata {
                    source_name: None, type_name: None, byte_range: Some((node.start_byte(), node.end_byte())), is_memory_state: false
                });

                let callee_role = self.spec.classify(callee_node.kind());
                if matches!(callee_role, NodeRole::Identifier) {
                    let func_name = self.source[callee_node.start_byte()..callee_node.end_byte()].to_string();
                    self.ir.push_instruction(self.current_block, Instruction::CallStatic {
                        dest: Some(dest),
                        mem_out: self.memory_var, // Generates new memory state
                        mem_in: self.memory_var,
                        func: func_name,
                        args,
                    });
                } else if let Some(LValue::Field { base, field }) = self.visit_lvalue(callee_node) {
                    self.ir.push_instruction(self.current_block, Instruction::CallVirtual {
                        dest: Some(dest),
                        mem_out: self.memory_var,
                        mem_in: self.memory_var,
                        method: field,
                        receiver: Operand::Var(base),
                        args,
                    });
                } else {
                    let func_op = self.visit_node(callee_node).unwrap_or(Operand::Unknown);
                    self.ir.push_instruction(self.current_block, Instruction::CallPointer {
                        dest: Some(dest),
                        mem_out: self.memory_var,
                        mem_in: self.memory_var,
                        func_ptr: func_op,
                        args,
                    });
                }
                
                Some(Operand::Var(dest))
            }
            
            // ─── CONTROL FLOW ────────────────────────────────────────────────────────
            NodeRole::WhileStatement { condition_field, body_field } => {
                let cond_node = node.child_by_field_name(condition_field)?;
                
                let loop_header = self.ir.new_block();
                let loop_body = self.ir.new_block();
                let loop_exit = self.ir.new_block();
                
                self.ir.set_terminator(self.current_block, Terminator::Jump(loop_header));
                self.ir.add_edge(self.current_block, loop_header);
                
                // Header (Evaluate Condition)
                self.current_block = loop_header;
                let cond_op = self.visit_node(cond_node).unwrap_or(Operand::Unknown);
                self.ir.set_terminator(self.current_block, Terminator::Branch {
                    cond: cond_op,
                    true_block: loop_body,
                    false_block: loop_exit,
                });
                self.ir.add_edge(self.current_block, loop_body);
                self.ir.add_edge(self.current_block, loop_exit);
                
                // Body
                self.current_block = loop_body;
                self.env.push(FxHashMap::default());
                if let Some(body) = node.child_by_field_name(body_field) {
                    self.visit_node(body);
                }
                self.ir.set_terminator(self.current_block, Terminator::Jump(loop_header));
                self.ir.add_edge(self.current_block, loop_header);
                self.env.pop();
                
                self.current_block = loop_exit;
                None
            }

            NodeRole::IfStatement { condition_field, consequence_field, alternative_field } => {
                let cond_node = node.child_by_field_name(condition_field)?;
                let cond_op = self.visit_node(cond_node).unwrap_or(Operand::Unknown);
                
                let true_block = self.ir.new_block();
                let false_block = self.ir.new_block();
                let merge_block = self.ir.new_block();
                
                let has_alt = node.child_by_field_name(alternative_field).is_some();
                let false_target = if has_alt { false_block } else { merge_block };
                
                self.ir.set_terminator(self.current_block, Terminator::Branch {
                    cond: cond_op,
                    true_block,
                    false_block: false_target,
                });
                self.ir.add_edge(self.current_block, true_block);
                self.ir.add_edge(self.current_block, false_target);
                
                // True Branch
                self.env.push(FxHashMap::default());
                self.current_block = true_block;
                if let Some(cons) = node.child_by_field_name(consequence_field) {
                    self.visit_node(cons);
                }
                self.ir.set_terminator(self.current_block, Terminator::Jump(merge_block));
                self.ir.add_edge(self.current_block, merge_block);
                self.env.pop();
                
                // False Branch
                if let Some(alt) = node.child_by_field_name(alternative_field) {
                    self.env.push(FxHashMap::default());
                    self.current_block = false_block;
                    self.visit_node(alt);
                    self.ir.set_terminator(self.current_block, Terminator::Jump(merge_block));
                    self.ir.add_edge(self.current_block, merge_block);
                    self.env.pop();
                }
                
                self.current_block = merge_block;
                None
            }
            
            // ─── PRIMITIVES ─────────────────────────────────────────────
            NodeRole::Identifier => Some(self.resolve_identifier(node)),
            NodeRole::LiteralString => Some(Operand::StringLiteral(self.source[node.start_byte()..node.end_byte()].to_string())),
            NodeRole::LiteralInteger => {
                let text = &self.source[node.start_byte()..node.end_byte()];
                if let Ok(i) = text.parse::<i64>() { Some(Operand::IntLiteral(i)) } else { Some(Operand::Unknown) }
            }
            
            _ => {
                let mut cursor = node.walk();
                let mut last_op = None;
                for child in node.children(&mut cursor) {
                    if child.is_named() {
                        if let Some(op) = self.visit_node(child) { last_op = Some(op); }
                    }
                }
                last_op
            }
        }
    }
}
