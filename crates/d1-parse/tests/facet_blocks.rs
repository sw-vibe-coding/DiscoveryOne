#[test]
fn dumps_facet_blocks() {
    let source = "*power(n e) -> (p) <-\n@front\n*p <- 1\n@left\n*p <- 2\n";
    let dump = d1_parse::parse_and_dump(source).expect("facet blocks should dump");
    assert_eq!(
        dump,
        "(module\n  (facet front\n    (signature power (inputs n e) (outputs p))\n    (assign (name p) (int 1)))\n  (facet left\n    (assign (name p) (int 2)))\n)\n"
    );
}
