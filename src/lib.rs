#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub genped_spec);

mod ast;

#[test]
fn expr_test() {
    let expr = genped_spec::SpecParser::new().parse("+").unwrap();
    // This fails, but that's fine.
    // This is just a hello world test.
    panic!("{:?}", expr);
}
