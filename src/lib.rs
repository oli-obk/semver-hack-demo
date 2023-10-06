pub fn entry_fn() -> Span {
    Span {
        data: "cake".into(),
        new: "boo".into(),
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Span {
    pub data: String,
    pub new: String,
}
