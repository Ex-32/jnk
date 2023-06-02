use jnk::context::MathContext;
use jnk::Integer;

#[test]
fn basic_eval() {
    let mut ctx = MathContext::new();
    let result = ctx.eval("((3 + 1 - 2) * 4 / 2) ^ 2").unwrap();
    let sixteen = Integer::from(16);

    assert_eq!(result, sixteen);
}
