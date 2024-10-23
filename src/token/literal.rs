#[derive(Debug, Clone)]
pub enum Literal {
    StringLiteral(String),
    NumberLiteral(f64),
    Null
}
