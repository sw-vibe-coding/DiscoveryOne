#[test]
fn parses_mint_init_assignment() {
    let module = d1_parse::parse("*n <- 0\n").expect("mint init should parse");
    assert_eq!(
        module,
        d1_parse::Module {
            facets: vec![d1_parse::Facet {
                aspect: d1_parse::Aspect::Front,
                stmts: vec![d1_parse::Stmt::Assign(
                    d1_parse::Pattern::Name("n".to_owned()),
                    d1_parse::Expr::Int(0)
                )],
            }],
        }
    );
}

#[test]
fn dumps_mint_init_assignment() {
    let dump = d1_parse::parse_and_dump("*n <- 0\n").expect("mint init should dump");
    assert_eq!(
        dump,
        "(module\n  (facet front\n    (assign (name n) (int 0)))\n)\n"
    );
}
