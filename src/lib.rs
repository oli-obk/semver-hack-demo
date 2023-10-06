pub fn entry_fn() -> Span {
    Span {
        data: "cake".into(),
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Span {
    data: String,
}
