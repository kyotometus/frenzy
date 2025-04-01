#[derive(Debug, Clone)]
pub enum Rate {
    Increment,
    Decrement,
    SetRate,
    CpsInputChanged(String),
}
