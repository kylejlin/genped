#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub slegen_spec);

mod ast;

#[test]
fn expr_test() {
    let expr = slegen_spec::SpecParser::new()
        .parse("struct struct")
        .expect("PARSE FAILED DUE TO SYNTAX ERROR");
    // This fails, but that's fine.
    // This is just a hello world test.
    panic!("\n\nSUCCESS\n\n{:?}", expr);
}
