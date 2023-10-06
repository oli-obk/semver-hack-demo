pub fn entry_fn() -> Span {
    let span = v2::entry_fn();
    Span { data: span.data }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Span {
    data: String,
}
