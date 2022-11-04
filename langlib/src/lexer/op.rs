#[derive(Debug, Clone, Eq, PartialEq)]

pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    EqSign,
}

#[derive(Debug, Clone, Eq, PartialEq)]

pub enum UnOp {
    Bang,
    Minus,
}
