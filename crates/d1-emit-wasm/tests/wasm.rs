#[test]
fn version_is_set() {
    assert!(!d1_emit_wasm::version().is_empty());
}

#[test]
fn runs_power_2_8() {
    let output = d1_emit_wasm::run_and_dump("*Power\n", &["n=2".to_owned(), "e=8".to_owned()])
        .expect("Power should run through WASM");
    assert_eq!(output, "256\n");
}
