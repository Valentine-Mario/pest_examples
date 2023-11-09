use std::ffi::CString;

pub enum MonadicVerb{
    Increment,
    Square,
    Negate,
    Reciprocal,
    Tally,
    Ceiling,
    ShapeOf,
}

pub enum DyadicVerb{
    Plus,
    Times,
    LessThan,
    LargerThan,
    Equal,
    Minus,
    Divide,
    Power,
    Residue,
    Copy,
    LargerOf,
    LargerOrEqual,
    Shape,
}

pub enum AstNode {
    Print(Box<AstNode>),
    Integer(i32),
    DoublePrecisionFloat(f64),
    MonadicOp{
        verb: MonadicVerb,
        expr: Box<AstNode>
    },
    DyadicOp{
        verb: DyadicVerb,
        lhs:Box<AstNode>,
        rhs:Box<AstNode>
    },
    Terms(Vec<AstNode>),
    IsGlobal{
        ident: String,
        expr: Box<AstNode>
    },
    Ident(String),
    Str(CString),
}