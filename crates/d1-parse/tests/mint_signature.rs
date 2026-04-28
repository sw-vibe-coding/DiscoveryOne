#[test]
fn parses_mint_signature() {
    let module = d1_parse::parse("*power(n e) -> (p) <-\n").expect("signature should parse");
    assert_eq!(
        module,
        d1_parse::Module {
            facets: vec![d1_parse::Facet {
                aspect: d1_parse::Aspect::Front,
                stmts: vec![d1_parse::Stmt::Signature(d1_parse::SigDecl {
                    name: "power".to_owned(),
                    inputs: vec![
                        d1_parse::Field {
                            name: "n".to_owned()
                        },
                        d1_parse::Field {
                            name: "e".to_owned()
                        },
                    ],
                    outputs: vec![d1_parse::Field {
                        name: "p".to_owned()
                    }],
                })],
            }],
        }
    );
}

#[test]
fn dumps_mint_signature() {
    let dump = d1_parse::parse_and_dump("*power(n e) -> (p) <-\n").expect("signature should dump");
    assert_eq!(
        dump,
        "(module\n  (facet front\n    (signature power (inputs n e) (outputs p)))\n)\n"
    );
}
