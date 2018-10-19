#[derive(Debug)]
pub enum Formula {
    Var(char),
    Op(Box<Formula>, Operator, Box<Formula>),
}

#[derive(Debug)]
pub enum Operator {
    And,
    Or,
}
