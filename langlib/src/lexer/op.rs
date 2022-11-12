#[derive(Debug, Clone, Eq, PartialEq)]

pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    EqSign,
    NeqSign,
    GreaterSign,
    LessSign,
    GreaterEqSign,
    LessEqSign,
    And,
    Or,
}

#[derive(Debug, Clone, Eq, PartialEq)]

pub enum UnOp {
    Bang,
    Minus,
}
