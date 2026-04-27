#[test]
fn dumps_arity_suffix_fixture() {
    let dump = d1_lex::dump_tokens(&d1_lex::lex("coord2 -> (x y)\n"));
    assert_eq!(
        dump,
        "IDENT  coord2\nRARROW\nLPAREN\nIDENT  x\nIDENT  y\nRPAREN\nEOF\n"
    );
}
