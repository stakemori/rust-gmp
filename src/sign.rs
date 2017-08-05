#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug, Serialize, Deserialize,)]
pub enum Sign {
    Negative,
    Zero,
    Positive,
}
