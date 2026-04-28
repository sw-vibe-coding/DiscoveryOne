#[test]
fn version_is_set() {
    assert!(!d1_interp::version().is_empty());
}

#[test]
fn runs_power_2_8() {
    let output = d1_interp::run_and_dump("*Power\n", &["n=2".to_owned(), "e=8".to_owned()])
        .expect("Power should run");
    assert_eq!(output, "256\n");
}
