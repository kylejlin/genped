#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub genped_spec);

mod ast;

#[test]
fn expr_test() {
    let expr = genped_spec::ExprParser::new()
        .parse("22 * 44 + 66")
        .unwrap();
    // This fails, but that's fine.
    // This is just a hello world test.
    assert_eq!(&format!("{:?}", expr), "((22 * 44) + 66)");
}
