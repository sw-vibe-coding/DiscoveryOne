#[test]
fn version_is_set() {
    assert!(!d1_check::version().is_empty());
}

#[test]
fn dumps_power_check_report() {
    let dump = d1_check::check_and_dump("*Power\n").expect("Power should check");
    assert_eq!(
        dump,
        "(checked-module\n  (definition Power\n    (facets front left right top bottom rear internal)\n    (rules R1 R3)\n    (warnings 0))\n)\n"
    );
}

#[test]
fn reports_left_arity_mismatch() {
    let err = d1_check::check_and_dump(
        "*LeftArityMismatch
  @front
    n e -> p

  @left
    n : Int
    e : Int
    extra : Int
",
    )
    .expect_err("left arity mismatch should fail");

    assert_eq!(
        err,
        "E007 left-facet input arity mismatch\n  at definition 'LeftArityMismatch', face Left, (x=0, y=6, z=0)\n  Front declares 2 inputs (n, e); Left lists 3 names."
    );
}

#[test]
fn reports_unbound_name() {
    let err = d1_check::check_and_dump(
        "*UnboundName
  @front
    n -> p
    p <- missing + n
",
    )
    .expect_err("unbound name should fail");

    assert_eq!(
        err,
        "E008 unbound name\n  at definition 'UnboundName', face Front, (x=9, y=3, z=0)\n  Name 'missing' is not bound by the front signature or prior assignments."
    );
}
