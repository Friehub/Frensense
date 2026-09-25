// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Phase 1: Intermediate Representation (IR)
//!
//! This module defines the 3-Address Code and Control Flow Graph structures.
//! It serves as the foundation for modern SSA-based dataflow analysis across
//! dynamic, object-oriented, and systems languages.
//!
//! NEW: Incorporates Memory SSA concepts. The heap is no longer a global state.
//! Instead, memory mutations generate a new `MemoryState` (represented as a VarId),
//! which perfectly tracks flow-sensitive object modifications over time.

use rustc_hash::FxHashMap;

/// A unique identifier for a variable or Memory State in the IR.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct VarId(pub usize);

/// A unique identifier for a Basic Block in the CFG.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct BlockId(pub usize);

/// Identifies the type of allocation for points-to analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum AllocationKind {
    Object,
    Array,
    ClassInstance(String),
}

/// Metadata mapping a VarId back to its original source code context.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct VarMetadata {
    pub source_name: Option<String>,
    pub type_name: Option<String>,
    pub byte_range: Option<(usize, usize)>,
    /// Flags if this VarId represents a hidden Memory State rather than a real code variable
    pub is_memory_state: bool,
    /// For a var produced by an object-literal composite: the keys whose
    /// values were merged into it (e.g. `["where", "message"]`). Sinks use
    /// this to classify the *shape* of the tainted argument (IDOR-style
    /// object payload vs raw injection string).
    pub object_keys: Vec<String>,
}

/// Operands are the inputs to instructions.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum Operand {
    Var(VarId),
    StringLiteral(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    BoolLiteral(bool),
    Null,
    Unknown,
}

/// A Phi node for SSA representation. Evaluated at the start of a block.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct Phi {
    pub dest: VarId,
    pub incoming: Vec<(BlockId, VarId)>,
}

/// A single linear instruction in 3-Address Code form.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum Instruction {
    /// v_dest = v_src
    Assign { dest: VarId, src: Operand },

    // --- Object & Array Memory Access (Memory SSA Enabled) ---
    // Reads depend on a specific moment in time (mem_in).
    // Writes consume a moment in time, and generate a new one (mem_out).
    /// v_dest = base.field  @ mem_in
    LoadField {
        dest: VarId,
        mem_in: VarId,
        base: VarId,
        field: String,
    },

    /// base.field = src  -> generates mem_out
    StoreField {
        mem_out: VarId,
        mem_in: VarId,
        base: VarId,
        field: String,
        src: Operand,
    },

    /// v_dest = base[index]  @ mem_in
    LoadElement {
        dest: VarId,
        mem_in: VarId,
        base: VarId,
        index: Operand,
    },

    /// base[index] = src  -> generates mem_out
    StoreElement {
        mem_out: VarId,
        mem_in: VarId,
        base: VarId,
        index: Operand,
        src: Operand,
    },

    // --- Global Environment ---
    /// v_dest = global_name
    LoadGlobal {
        dest: VarId,
        mem_in: VarId,
        name: String,
    },

    /// global_name = src
    StoreGlobal {
        mem_out: VarId,
        mem_in: VarId,
        name: String,
        src: Operand,
    },

    // --- Functions & Calls ---
    // Function calls can mutate memory, so they consume mem_in and return mem_out.
    /// mem_out, dest = func(args) @ mem_in
    CallStatic {
        dest: Option<VarId>,
        mem_out: VarId,
        mem_in: VarId,
        func: String,
        args: Vec<Operand>,
    },

    /// mem_out, dest = receiver.method(args) @ mem_in
    CallVirtual {
        dest: Option<VarId>,
        mem_out: VarId,
        mem_in: VarId,
        method: String,
        receiver: Operand,
        args: Vec<Operand>,
    },

    /// mem_out, dest = func_ptr(args) @ mem_in
    CallPointer {
        dest: Option<VarId>,
        mem_out: VarId,
        mem_in: VarId,
        func_ptr: Operand,
        args: Vec<Operand>,
    },

    // --- Memory & Pointers (Systems Languages) ---
    /// mem_out, v_dest = new Object/Array() @ mem_in
    Allocate {
        dest: VarId,
        mem_out: VarId,
        mem_in: VarId,
        kind: AllocationKind,
    },

    /// v_dest = &src
    AddressOf { dest: VarId, src: VarId },

    /// v_dest = *ptr @ mem_in
    Dereference {
        dest: VarId,
        mem_in: VarId,
        ptr: Operand,
    },

    // --- Types & Multiple Returns ---
    Cast {
        dest: VarId,
        src: Operand,
        target_type: String,
    },
    ExtractValue {
        dest: VarId,
        tuple: Operand,
        index: usize,
    },

    // --- Operations & Async ---
    BinaryOp {
        dest: VarId,
        op: String,
        lhs: Operand,
        rhs: Operand,
    },
    UnaryOp {
        dest: VarId,
        op: String,
        src: Operand,
    },
    Await {
        dest: VarId,
        mem_out: VarId,
        mem_in: VarId,
        promise: Operand,
    },
    Yield {
        dest: Option<VarId>,
        mem_out: VarId,
        mem_in: VarId,
        src: Option<Operand>,
    },
}

/// Determines how control flow exits a BasicBlock.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum Terminator {
    Jump(BlockId),
    Branch {
        cond: Operand,
        true_block: BlockId,
        false_block: BlockId,
    },
    Switch {
        cond: Operand,
        cases: Vec<(Operand, BlockId)>,
        default_block: BlockId,
    },
    Return {
        src: Option<Operand>,
    },
    Throw {
        src: Operand,
    },
    Unreachable,
    None,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct BasicBlock {
    pub id: BlockId,
    pub phis: Vec<Phi>,
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
    pub predecessors: Vec<BlockId>,
    pub successors: Vec<BlockId>,
    pub unwind_to: Option<BlockId>,
}

impl BasicBlock {
    pub fn new(id: BlockId) -> Self {
        Self {
            id,
            phis: Vec::new(),
            instructions: Vec::new(),
            terminator: Terminator::None,
            predecessors: Vec::new(),
            successors: Vec::new(),
            unwind_to: None,
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionIR {
    pub name: String,
    pub parameters: Vec<VarId>,
    /// The initial Memory State parameter for the function (representing the heap upon entry)
    pub initial_memory_state: VarId,
    pub blocks: FxHashMap<BlockId, BasicBlock>,
    pub entry_block: BlockId,
    pub var_metadata: FxHashMap<VarId, VarMetadata>,
    #[cfg_attr(feature = "serialize", serde(default))]
    // legacy safety: recompute via max ids if absent
    next_var_id: usize,
    #[cfg_attr(feature = "serialize", serde(default))]
    next_block_id: usize,
}

impl FunctionIR {
    pub fn new(name: String) -> Self {
        let entry_id = BlockId(0);
        let mut blocks = FxHashMap::default();
        blocks.insert(entry_id, BasicBlock::new(entry_id));

        let mut f = Self {
            name,
            parameters: Vec::new(),
            initial_memory_state: VarId(0), // Placeholder
            blocks,
            entry_block: entry_id,
            var_metadata: FxHashMap::default(),
            next_var_id: 1,
            next_block_id: 1,
        };

        // Allocate the hidden Memory State parameter
        f.initial_memory_state = f.new_var(VarMetadata {
            source_name: Some("InitialHeapState".into()),
            type_name: None,
            byte_range: None,
            is_memory_state: true,
            object_keys: Vec::new(),
        });

        f
    }

    pub fn new_var(&mut self, metadata: VarMetadata) -> VarId {
        let id = VarId(self.next_var_id);
        self.next_var_id += 1;
        self.var_metadata.insert(id, metadata);
        id
    }

    pub fn new_block(&mut self) -> BlockId {
        let id = BlockId(self.next_block_id);
        self.next_block_id += 1;
        self.blocks.insert(id, BasicBlock::new(id));
        id
    }

    pub fn push_phi(&mut self, block: BlockId, phi: Phi) {
        if let Some(b) = self.blocks.get_mut(&block) {
            b.phis.push(phi);
        }
    }

    pub fn push_instruction(&mut self, block: BlockId, instr: Instruction) {
        if let Some(b) = self.blocks.get_mut(&block) {
            b.instructions.push(instr);
        }
    }

    pub fn set_terminator(&mut self, block: BlockId, terminator: Terminator) {
        if let Some(b) = self.blocks.get_mut(&block) {
            b.terminator = terminator;
        }
    }

    pub fn add_edge(&mut self, from: BlockId, to: BlockId) {
        if let Some(f) = self.blocks.get_mut(&from)
            && !f.successors.contains(&to)
        {
            f.successors.push(to);
        }
        if let Some(t) = self.blocks.get_mut(&to)
            && !t.predecessors.contains(&from)
        {
            t.predecessors.push(from);
        }
    }
}
