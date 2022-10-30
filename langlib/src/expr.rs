use super::lexer::op::Op;

#[derive(Debug, Clone, Eq, PartialEq)]

pub enum Expr {
    NumExpr(NumExpr),
    Num(i32),
    Str(String),
    Bool(bool),
    Comparison(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn eval(&self) -> bool {
        match self {
            Expr::NumExpr(expr) => expr.eval() > 0,
            Expr::Num(num) => *num > 0,
            Expr::Str(str) => !str.is_empty(),
            Expr::Bool(boolean) => *boolean,

            Expr::Comparison(a, b) => match (a.as_ref(), b.as_ref()) {
                (Expr::NumExpr(a), Expr::NumExpr(b)) => a.eval() == b.eval(),
                (Expr::Num(a), Expr::Num(b)) => a == b,
                (Expr::Str(a), Expr::Str(b)) => a == b,
                (Expr::Bool(a), Expr::Bool(b)) => a == b,
                _ => false,
            },
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NumExpr {
    l: i32,
    r: i32,
    op: Op,
}

impl NumExpr {
    pub fn new(left: i32, right: i32, op: Op) -> Self {
        Self {
            l: left,
            r: right,
            op,
        }
    }

    pub fn eval(&self) -> i32 {
        match self.op {
            Op::Add => self.l + self.r,
            Op::Sub => self.l - self.r,
            Op::Mul => self.l * self.r,
            Op::Div => self.l / self.r,
        }
    }
}
