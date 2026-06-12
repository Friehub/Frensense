// SPDX-License-Identifier: MIT

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AbstractKind {
    FunctionDef,
    ClassDef,
    MethodDef,
    InterfaceDef,
    StructDef,
    EnumDef,
    ConstDef,
    ModuleDef,

    Call,
    MethodCall,
    Await,
    Return,
    Assign,
    BinaryOp,
    UnaryOp,
    Conditional,
    Loop,
    Match,
    Closure,
    TryCatch,
    Throw,
    Unsafe,
    Async,

    StringLiteral,
    NumberLiteral,
    BoolLiteral,
    Identifier,
    TypeAnnotation,

    Block,
    Parameters,
    Arguments,
    ImportDecl,
    ExportDecl,

    Other,
}
