use semver_hack_demo::entry_fn;

#[test]
fn foo() {
    assert_eq!(entry_fn(), entry_fn());
    assert_eq!(format!("{:?}", entry_fn()), "Span { data: \"cake\" }")
}
