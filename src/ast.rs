#[derive(Debug)]
pub enum Formula {
    Var(char),
    BinaryOp(Box<Formula>, BinaryOperator, Box<Formula>),
    MonaryOp(MonaryOperator, Box<Formula>),
}

#[derive(Debug)]
pub enum BinaryOperator {
    And,
    Or,
}

#[derive(Debug)]
pub enum MonaryOperator {
    Not,
}
