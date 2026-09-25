// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Phase 1.5: AST Lowering Pass
//!
//! This module traverses the `tree-sitter` AST using `frensense-lang` and
//! builds the Unstructured FunctionIR.
//!
//! NEW: It seamlessly integrates Memory SSA. The entire Heap is treated as a
//! single mutable variable `self.memory_var` during lowering. The SSA Builder
//! will automatically split this into `mem_1`, `mem_2` and generate Memory Phis!

use crate::ir::function::*;
use frensense_lang::{LanguageSpec, NodeRole};
use rustc_hash::FxHashMap;
use tree_sitter::Node;

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

        let ctx = Self {
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
            object_keys: Vec::new(),
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
            // MemberAccess covers both `obj.field` and `obj[i]`: JS maps
            // subscript_expression here with property_field = "index". Distinguish
            // by attempting to evaluate the property node as an operand.
            NodeRole::MemberAccess {
                object_field,
                property_field,
            } => {
                let obj_node = node.child_by_field_name(object_field)?;
                let prop_node = node.child_by_field_name(property_field)?;

                let base_op = self.visit_node(obj_node)?;

                if let Operand::Var(base_var) = base_op {
                    match self.visit_node(prop_node) {
                        // Index-like (identifier/expression resolving to a var):
                        // prefer element semantics for lvalues. Property-name
                        // decision is language-driven (spec.is_property_kind):
                        // Python attributes are plain `identifier`, Go/Rust use
                        // `field_identifier`, substring guessing broke all
                        // three into LoadElement/CallPointer, losing sink names.
                        Some(Operand::Var(index_var)) => {
                            if self.spec.is_property_kind(prop_node.kind()) {
                                let field_name = self.source
                                    [prop_node.start_byte()..prop_node.end_byte()]
                                    .to_string();
                                Some(LValue::Field {
                                    base: base_var,
                                    field: field_name,
                                })
                            } else {
                                Some(LValue::Element {
                                    base: base_var,
                                    index: Operand::Var(index_var),
                                })
                            }
                        }
                        _ => {
                            let field_name = self.source
                                [prop_node.start_byte()..prop_node.end_byte()]
                                .to_string();
                            Some(LValue::Field {
                                base: base_var,
                                field: field_name,
                            })
                        }
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn visit_node(&mut self, node: Node) -> Option<Operand> {
        let role = self.spec.classify(node.kind());

        match role {
            // ─── MEMORY & VARIABLES ────────────────────────────────────────────────
            NodeRole::Declaration {
                name_field,
                value_field,
            } => {
                // `const name = value;`, bind the name in the current scope.
                // If the node itself carries a name (variable_declarator),
                // lower the value and record the binding. Container statements
                // (lexical_declaration) have no name/value fields and fall
                // through to the child walk, which re-enters here per
                // declarator.
                let value_node = node.child_by_field_name(value_field);
                let name_node = node.child_by_field_name(name_field);
                // Go/Python wrap names in a list node (`expression_list`,
                // `identifier_list`): unwrap single-element lists so the
                // simple-binding arm fires instead of the container walk,
                // which never binds the name and leaves uses pointing at a
                // def-less phantom variable.
                let name_node = name_node.map(|n| {
                    if matches!(
                        n.kind(),
                        "expression_list" | "identifier_list" | "expression_sequence"
                    ) {
                        let mut nc = n.walk();
                        let named: Vec<_> = n.named_children(&mut nc).collect();
                        if named.len() == 1 { named[0] } else { n }
                    } else {
                        n
                    }
                });
                match (name_node, value_node) {
                    (Some(name_node), Some(value_node)) if name_node.kind() == "identifier" => {
                        // Simple `const name = value`.
                        let rhs_op = self.visit_node(value_node).unwrap_or(Operand::Unknown);
                        if let Operand::Var(dest) = self.resolve_identifier(name_node) {
                            self.ir.push_instruction(
                                self.current_block,
                                Instruction::Assign { dest, src: rhs_op },
                            );
                        }
                        None
                    }
                    (Some(pattern), Some(value_node))
                        if pattern.kind() == "object_pattern"
                            || pattern.kind() == "array_pattern" =>
                    {
                        // Destructuring: `const { a: x, b } = obj`.
                        // Lower the RHS first (it defines the base value).
                        let base_op = self.visit_node(value_node);
                        if let Some(Operand::Var(base)) = base_op {
                            let mut pc = pattern.walk();
                            for sub in pattern.named_children(&mut pc) {
                                // `key: sub_name` or shorthand `name`.
                                let (prop, target) = if sub.kind() == "pair_pattern" {
                                    let k = sub.child_by_field_name("key");
                                    let v = sub.child_by_field_name("value");
                                    match (k, v) {
                                        (Some(k), Some(v)) => (Some(k), Some(v)),
                                        _ => (None, None),
                                    }
                                } else {
                                    (Some(sub), Some(sub))
                                };
                                if let (Some(k), Some(v)) = (prop, target) {
                                    let field =
                                        self.source[k.start_byte()..k.end_byte()].to_string();
                                    if let Operand::Var(dest) = self.resolve_identifier(v) {
                                        self.ir.push_instruction(
                                            self.current_block,
                                            Instruction::LoadField {
                                                dest,
                                                mem_in: self.memory_var,
                                                base,
                                                field,
                                            },
                                        );
                                    }
                                }
                            }
                        }
                        None
                    }
                    _ => {
                        // Container: lower children (each declarator hits the
                        // (name, value) arm above).
                        let mut cursor = node.walk();
                        for child in node.children(&mut cursor) {
                            if child.is_named() {
                                self.visit_node(child);
                            }
                        }
                        None
                    }
                }
            }
            NodeRole::Assignment {
                lhs_field,
                rhs_field,
            } => {
                let left_node = node.child_by_field_name(lhs_field)?;
                let right_node = node.child_by_field_name(rhs_field)?;

                let rhs_op = self.visit_node(right_node).unwrap_or(Operand::Unknown);
                let lval = self.visit_lvalue(left_node);

                match lval {
                    Some(LValue::Variable(dest)) => {
                        self.ir.push_instruction(
                            self.current_block,
                            Instruction::Assign { dest, src: rhs_op },
                        );
                    }
                    Some(LValue::Field { base, field }) => {
                        // Memory Mutation! Consumes self.memory_var, and re-assigns it.
                        self.ir.push_instruction(
                            self.current_block,
                            Instruction::StoreField {
                                mem_out: self.memory_var,
                                mem_in: self.memory_var,
                                base,
                                field,
                                src: rhs_op,
                            },
                        );
                    }
                    Some(LValue::Element { base, index }) => {
                        self.ir.push_instruction(
                            self.current_block,
                            Instruction::StoreElement {
                                mem_out: self.memory_var,
                                mem_in: self.memory_var,
                                base,
                                index,
                                src: rhs_op,
                            },
                        );
                    }
                    None => {}
                }
                None
            }
            NodeRole::MemberAccess {
                object_field,
                property_field,
            } => {
                // r-value member/element read: `obj.field` or `obj[i]`.
                let obj_node = node.child_by_field_name(object_field)?;
                let prop_node = node.child_by_field_name(property_field)?;

                let base_op = self.visit_node(obj_node)?;

                if let Operand::Var(base_var) = base_op {
                    let dest = self.ir.new_var(VarMetadata {
                        source_name: None,
                        type_name: None,
                        // Span needed: sink findings resolve file/line from
                        // the tainted operand's byte_range. Without it, a
                        // member read passed directly to a sink reports line 0.
                        byte_range: Some((node.start_byte(), node.end_byte())),
                        is_memory_state: false,
                        object_keys: Vec::new(),
                    });
                    match self.visit_node(prop_node) {
                        Some(Operand::Var(index_var))
                            if !self.spec.is_property_kind(prop_node.kind()) =>
                        {
                            // Subscript with a computed index.
                            self.ir.push_instruction(
                                self.current_block,
                                Instruction::LoadElement {
                                    dest,
                                    mem_in: self.memory_var,
                                    base: base_var,
                                    index: Operand::Var(index_var),
                                },
                            );
                        }
                        _ => {
                            let field_name = self.source
                                [prop_node.start_byte()..prop_node.end_byte()]
                                .to_string();
                            self.ir.push_instruction(
                                self.current_block,
                                Instruction::LoadField {
                                    dest,
                                    mem_in: self.memory_var,
                                    base: base_var,
                                    field: field_name,
                                },
                            );
                        }
                    }
                    Some(Operand::Var(dest))
                } else {
                    None
                }
            }

            // ─── CALLS (Static, Virtual, Pointer) ─────────────────────────────────────
            NodeRole::Call {
                callee_field,
                args_field,
            } => {
                let callee_node = node.child_by_field_name(callee_field)?;
                // Some grammars don't give the args child a field name
                // (tree-sitter-rust `macro_invocation` has `macro` but
                // `token_tree` is fieldless), fall back to the last named
                // child, which is the argument container in every grammar we
                // ship.
                let args_node = match node.child_by_field_name(args_field) {
                    Some(a) => Some(a),
                    None => {
                        let mut fc = node.walk();
                        let kids: Vec<_> =
                            node.children(&mut fc).filter(|c| c.is_named()).collect();
                        kids.last().copied()
                    }
                }?;

                let mut args = Vec::new();
                let mut cursor = args_node.walk();
                for child in args_node.children(&mut cursor) {
                    if child.is_named()
                        && let Some(op) = self.visit_node(child)
                    {
                        args.push(op);
                    }
                }

                let dest = self.ir.new_var(VarMetadata {
                    source_name: None,
                    type_name: None,
                    byte_range: Some((node.start_byte(), node.end_byte())),
                    is_memory_state: false,
                    object_keys: Vec::new(),
                });

                let callee_role = self.spec.classify(callee_node.kind());
                if matches!(callee_role, NodeRole::Identifier) {
                    let func_name =
                        self.source[callee_node.start_byte()..callee_node.end_byte()].to_string();
                    self.ir.push_instruction(
                        self.current_block,
                        Instruction::CallStatic {
                            dest: Some(dest),
                            mem_out: self.memory_var, // Generates new memory state
                            mem_in: self.memory_var,
                            func: func_name,
                            args,
                        },
                    );
                } else if let Some(LValue::Field { base, field }) = self.visit_lvalue(callee_node) {
                    self.ir.push_instruction(
                        self.current_block,
                        Instruction::CallVirtual {
                            dest: Some(dest),
                            mem_out: self.memory_var,
                            mem_in: self.memory_var,
                            method: field,
                            receiver: Operand::Var(base),
                            args,
                        },
                    );
                } else {
                    let func_op = self.visit_node(callee_node).unwrap_or(Operand::Unknown);
                    self.ir.push_instruction(
                        self.current_block,
                        Instruction::CallPointer {
                            dest: Some(dest),
                            mem_out: self.memory_var,
                            mem_in: self.memory_var,
                            func_ptr: func_op,
                            args,
                        },
                    );
                }

                Some(Operand::Var(dest))
            }

            // ─── CONTROL FLOW ────────────────────────────────────────────────────────
            // Loops: `while_statement`, `for_statement`, `do_statement`, … all
            // classify as NodeRole::Loop. The condition is the first child that
            // is not the body; the body is the parenthesized statement/block.
            NodeRole::Loop => {
                // Find condition and body children by grammar convention.
                let mut cursor = node.walk();
                let children: Vec<Node> = node
                    .children(&mut cursor)
                    .filter(|c| c.is_named())
                    .collect();

                // Heuristic across grammars: the last named child that is a
                // statement/block is the body; the condition is the operand
                // expression just before it (for `for` loops this is coarse,
                // acceptable: the condition only affects branch pruning).
                let body = children
                    .iter()
                    .rev()
                    .find(|c| {
                        let k = c.kind();
                        k.contains("statement") || k == "block" || k.contains("block")
                    })
                    .copied();
                let cond = children
                    .iter()
                    .find(|c| {
                        let k = c.kind();
                        !k.contains("statement")
                            && k != "block"
                            && !k.contains("block")
                            && !k.contains("declarator")
                            && !k.contains("update")
                            && !k.contains("initializer")
                    })
                    .copied();

                let loop_header = self.ir.new_block();
                let loop_body = self.ir.new_block();
                let loop_exit = self.ir.new_block();

                self.ir
                    .set_terminator(self.current_block, Terminator::Jump(loop_header));
                self.ir.add_edge(self.current_block, loop_header);

                // Header (Evaluate Condition)
                self.current_block = loop_header;
                let cond_op = cond
                    .and_then(|c| self.visit_node(c))
                    .unwrap_or(Operand::Unknown);
                self.ir.set_terminator(
                    self.current_block,
                    Terminator::Branch {
                        cond: cond_op,
                        true_block: loop_body,
                        false_block: loop_exit,
                    },
                );
                self.ir.add_edge(self.current_block, loop_body);
                self.ir.add_edge(self.current_block, loop_exit);

                // Body
                self.current_block = loop_body;
                self.env.push(FxHashMap::default());
                if let Some(body) = body {
                    self.visit_node(body);
                }
                self.ir
                    .set_terminator(self.current_block, Terminator::Jump(loop_header));
                self.ir.add_edge(self.current_block, loop_header);
                self.env.pop();

                self.current_block = loop_exit;
                None
            }

            NodeRole::Branch
                if node.kind() == "ternary_expression"
                    || node.kind() == "conditional_expression"
                    || node.kind() == "conditional_type" =>
            {
                // Ternary used as a VALUE: `x = cond ? a : b`.
                // The Branch statement handler drops the expression's value
                // (its children are expressions, not statement blocks), which
                // severed taint through every `a ? b : c` initializer, the
                // #1 recall gap in the Juice Shop baseline (search.ts SQLi).
                //
                // Lowered STRAIGHT-LINE (no CFG split): both arms are visited
                // in the current block and the result is an Assign from the
                // tainted-capable arm. Sound for taint, a may-analysis over
                // paths, so the union of both arms' values is exactly right.
                // A CFG/phi lowering was tried first but pre-filled phi
                // `incoming` entries hold pre-SSA var ids; the SSA renamer
                // fills its own phis and rewires only its own, so hand-built
                // phis silently lose their edges after renaming.
                let mut cursor = node.walk();
                let children: Vec<Node> = node
                    .children(&mut cursor)
                    .filter(|c| c.is_named() && c.kind() != "comment")
                    .collect();
                // JS grammar: [condition, consequence, alternative]
                if children.len() != 3 {
                    // Malformed ternary: fall through to generic child walk.
                    let mut c = node.walk();
                    let mut last_op = None;
                    for child in node.children(&mut c) {
                        if child.is_named()
                            && let Some(op) = self.visit_node(child)
                        {
                            last_op = Some(op);
                        }
                    }
                    return last_op;
                }

                // Condition and both arms evaluated in the current block.
                // Side effects in the condition/arms still lower to their
                // instructions; only control-flow precision is lost.
                let _ = self.visit_node(children[0]).unwrap_or(Operand::Unknown);
                let true_val = self.visit_node(children[1]);
                let false_val = self.visit_node(children[2]);

                // Merge: prefer a fresh var assigned from the first var-valued
                // arm (preserves one def site for SVFG edges). A literal arm
                // carries no taint, so the var arm dominates the may-analysis.
                match (true_val, false_val) {
                    (Some(Operand::Var(tv)), Some(Operand::Var(fv))) => {
                        let dest = self.ir.new_var(VarMetadata {
                            source_name: None,
                            type_name: None,
                            byte_range: Some((node.start_byte(), node.end_byte())),
                            is_memory_state: false,
                            object_keys: Vec::new(),
                        });
                        self.ir.push_instruction(
                            self.current_block,
                            Instruction::Assign {
                                dest,
                                src: Operand::Var(tv),
                            },
                        );
                        self.ir.push_instruction(
                            self.current_block,
                            Instruction::Assign {
                                dest,
                                src: Operand::Var(fv),
                            },
                        );
                        Some(Operand::Var(dest))
                    }
                    // One arm is a var, other is a constant: the var survives;
                    // only the tainted arm matters for alerts.
                    (Some(Operand::Var(v)), Some(_)) | (Some(_), Some(Operand::Var(v))) => {
                        Some(Operand::Var(v))
                    }
                    (Some(v), None) | (None, Some(v)) => Some(v),
                    (None, None) => None,
                    // Both constants: fold to the first, neither carries taint.
                    (a, b) => a.or(b),
                }
            }

            NodeRole::Branch => {
                // if / switch statements. Grammar fields differ per language;
                // walk named children: condition = first expression-ish child,
                // consequence = first block/statement, alternative = `else`
                // clause if present.
                let mut cursor = node.walk();
                let children: Vec<Node> = node
                    .children(&mut cursor)
                    .filter(|c| c.is_named())
                    .collect();

                let cond_node = children
                    .iter()
                    .find(|c| {
                        let k = c.kind();
                        !k.contains("statement") && k != "block" && k != "else_clause"
                    })
                    .copied();
                let cons_node = children
                    .iter()
                    .find(|c| {
                        let k = c.kind();
                        k.contains("statement") || k.contains("block")
                    })
                    .copied();
                let alt_node = children.iter().find(|c| c.kind().contains("else")).copied();

                let cond_op = cond_node
                    .and_then(|c| self.visit_node(c))
                    .unwrap_or(Operand::Unknown);

                let true_block = self.ir.new_block();
                let false_block = self.ir.new_block();
                let merge_block = self.ir.new_block();

                let false_target = if alt_node.is_some() {
                    false_block
                } else {
                    merge_block
                };

                self.ir.set_terminator(
                    self.current_block,
                    Terminator::Branch {
                        cond: cond_op,
                        true_block,
                        false_block: false_target,
                    },
                );
                self.ir.add_edge(self.current_block, true_block);
                self.ir.add_edge(self.current_block, false_target);

                // True Branch
                self.env.push(FxHashMap::default());
                self.current_block = true_block;
                if let Some(cons) = cons_node {
                    self.visit_node(cons);
                }
                self.ir
                    .set_terminator(self.current_block, Terminator::Jump(merge_block));
                self.ir.add_edge(self.current_block, merge_block);
                self.env.pop();

                // False Branch
                if let Some(alt) = alt_node {
                    self.env.push(FxHashMap::default());
                    self.current_block = false_block;
                    self.visit_node(alt);
                    self.ir
                        .set_terminator(self.current_block, Terminator::Jump(merge_block));
                    self.ir.add_edge(self.current_block, merge_block);
                    self.env.pop();
                }

                self.current_block = merge_block;
                None
            }

            NodeRole::Return => {
                // return expr; → terminator. The value (if any) becomes the
                // return operand; `as`-casts resolve through the fallback
                // child walk since NodeRole has no Cast shape.
                let mut cursor = node.walk();
                let value: Option<Operand> = node
                    .named_children(&mut cursor)
                    .find(|c| c.kind() != "comment")
                    .and_then(|c| self.visit_node(c));
                self.ir
                    .set_terminator(self.current_block, Terminator::Return { src: value });
                None
            }

            // ─── OPERATORS ───────────────────────────────────────────────
            // `a + b`, `` `x ${y} z` `` (JS lowers interpolation to a binary
            // concat shape), `a || b`, every operand contributes to the
            // result value. Previously these fell into the generic fallback,
            // which returns the LAST child operand and silently dropped taint
            // from earlier ones, the #1 recall gap in the e2e report.
            NodeRole::BinaryOp => {
                let mut operands: Vec<Operand> = Vec::new();
                let mut operator: Option<String> = None;
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if !child.is_named() {
                        // The operator token (+, -, ||, ...) sits between
                        // the operand children.
                        let text = self.source[child.start_byte()..child.end_byte()].trim();
                        if !text.is_empty() && operator.is_none() {
                            operator = Some(text.to_string());
                        }
                        continue;
                    }
                    if child.kind() == "comment" {
                        continue;
                    }
                    if let Some(op) = self.visit_node(child) {
                        operands.push(op);
                    }
                }
                if operands.len() == 1 {
                    // Degenerate single-operand form, pass through.
                    return Some(operands.pop().unwrap());
                }
                if operands.is_empty() {
                    return Some(Operand::Unknown);
                }
                // String concatenation is semantically significant for taint
                // (`"...' + id + "'"` builds a query); record the operator so
                // later constraint/value tracking can classify the shape.
                let op_name = match operator.as_deref() {
                    Some("+") | None => "concat".to_string(),
                    Some(o) => o.to_string(),
                };
                // Fold operands left-associative: ((a op b) op c).
                let mut it = operands.into_iter();
                let mut acc = it.next().unwrap();
                for rhs in it {
                    let next_dest = self.ir.new_var(VarMetadata {
                        source_name: None,
                        type_name: None,
                        byte_range: Some((node.start_byte(), node.end_byte())),
                        is_memory_state: false,
                        object_keys: Vec::new(),
                    });
                    self.ir.push_instruction(
                        self.current_block,
                        Instruction::BinaryOp {
                            dest: next_dest,
                            op: op_name.clone(),
                            lhs: acc,
                            rhs: rhs.clone(),
                        },
                    );
                    acc = Operand::Var(next_dest);
                }
                Some(acc)
            }
            NodeRole::UnaryOp => {
                let mut cursor = node.walk();
                let operand = node
                    .children(&mut cursor)
                    .find(|c| c.is_named() && c.kind() != "comment")
                    .and_then(|c| self.visit_node(c))
                    .unwrap_or(Operand::Unknown);
                let dest = self.ir.new_var(VarMetadata {
                    source_name: None,
                    type_name: None,
                    byte_range: Some((node.start_byte(), node.end_byte())),
                    is_memory_state: false,
                    object_keys: Vec::new(),
                });
                self.ir.push_instruction(
                    self.current_block,
                    Instruction::UnaryOp {
                        dest,
                        op: "neg".into(),
                        src: operand,
                    },
                );
                Some(Operand::Var(dest))
            }

            // ─── PRIMITIVES ─────────────────────────────────────────────
            NodeRole::Identifier => Some(self.resolve_identifier(node)),
            NodeRole::Other if node.kind() == "as_expression" => {
                // TS cast `expr as T`, transparent for value flow.
                let mut cursor = node.walk();
                let mut first = None;
                for child in node.children(&mut cursor) {
                    if child.is_named() && child.kind() != "comment" && first.is_none() {
                        first = self.visit_node(child);
                    }
                }
                first
            }
            NodeRole::Literal
                if matches!(
                    node.kind(),
                    "template_string" | "string" | "binary_expression"
                ) =>
            {
                // Template literals / string concat: the whole expression is a
                // value that flows. Walk children so interpolations
                // (template_substitution) and concatenated operands
                // (binary_expression) contribute their operands, taint flows
                // through `${x}` and `a + x`. The literal *text* fragments are
                // strings themselves; return the LAST evaluated child as the
                // expression value when present.
                let mut cursor = node.walk();
                let mut last_op = None;
                for child in node.children(&mut cursor) {
                    if child.is_named()
                        && child.kind() != "string_fragment"
                        && let Some(op) = self.visit_node(child)
                    {
                        last_op = Some(op);
                    }
                }
                match last_op {
                    Some(op) => Some(op),
                    None => {
                        let text = &self.source[node.start_byte()..node.end_byte()];
                        Some(Operand::StringLiteral(text.to_string()))
                    }
                }
            }
            NodeRole::Literal => {
                let text = &self.source[node.start_byte()..node.end_byte()];
                if let Ok(i) = text.parse::<i64>() {
                    Some(Operand::IntLiteral(i))
                } else {
                    Some(Operand::StringLiteral(text.to_string()))
                }
            }

            NodeRole::Composite => {
                // Value-context composite literal: `{ key: val }`, `[a, b]`,
                // spread payloads. Every child is a value-producer, so visit
                // them ALL and merge: any tainted child taints the composite.
                //
                // The previous generic fallback returned the LAST child's
                // operand, which silently dropped taint from earlier children:
                // `pool.query({ ...req.body, sql: "SELECT 1" })` collapsed to
                // the `"SELECT 1"` literal. Taint is a may-analysis, so the
                // conservative merge (any child → whole composite) is sound.
                //
                // A fresh var is minted for multi-child composites so the
                // SVFG gets ONE def site aggregating all child flows; single-
                // child composites (parenthesized exprs) stay transparent.
                // Pair keys (`pair` = `key: value` inside an object literal)
                // are remembered on the composite's var so sinks can classify
                // the argument *shape* (object payload vs raw value).
                let mut cursor = node.walk();
                let child_ops: Vec<Operand> = node
                    .children(&mut cursor)
                    .filter(|c| c.is_named() && c.kind() != "comment")
                    .filter_map(|c| self.visit_node(c))
                    .collect();
                // Pair keys (`pair` = `key: value` inside an object literal)
                // are remembered on the composite's var so sinks can classify
                // the argument *shape* (object payload vs raw value).
                let mut pair_keys: Vec<String> = Vec::new();
                let mut kcursor = node.walk();
                for c in node.children(&mut kcursor) {
                    if c.is_named()
                        && c.kind() == "pair"
                        && let Some(key) = c.child_by_field_name("key")
                    {
                        let ktext = &self.source[key.start_byte()..key.end_byte()];
                        pair_keys
                            .push(ktext.trim_matches(|ch| ch == '"' || ch == '\'').to_string());
                    }
                }
                let var_ops: Vec<Operand> = child_ops
                    .iter()
                    .filter(|op| matches!(op, Operand::Var(_)))
                    .cloned()
                    .collect();
                match var_ops.len() {
                    0 => child_ops.into_iter().next(),
                    // Single var child: collapse transparently, but carry the
                    // pair keys over so shape classification still works.
                    1 => {
                        if let Some(Operand::Var(v)) = var_ops.first()
                            && let Some(meta) = self.ir.var_metadata.get_mut(v)
                        {
                            meta.object_keys = pair_keys;
                        }
                        var_ops.into_iter().next()
                    }
                    _ => {
                        let dest = self.ir.new_var(VarMetadata {
                            source_name: None,
                            type_name: None,
                            byte_range: Some((node.start_byte(), node.end_byte())),
                            is_memory_state: false,
                            object_keys: pair_keys,
                        });
                        for op in var_ops {
                            self.ir.push_instruction(
                                self.current_block,
                                Instruction::Assign { dest, src: op },
                            );
                        }
                        Some(Operand::Var(dest))
                    }
                }
            }

            _ => {
                let mut cursor = node.walk();
                let mut last_op = None;
                for child in node.children(&mut cursor) {
                    if child.is_named()
                        && let Some(op) = self.visit_node(child)
                    {
                        last_op = Some(op);
                    }
                }
                last_op
            }
        }
    }
}
