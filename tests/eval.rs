use jnk::context::MathContext;
use jnk::Integer;

#[test]
fn basic_eval() {
    let mut ctx = MathContext::new();

    let result = ctx.eval("((3 + 1 - 2) * 4 / 2) ^ 2").unwrap();
    let sixteen = Integer::from(16);
    assert_eq!(result.value, sixteen);
}

#[test]
fn env_eval() {
    let mut ctx = MathContext::new();

    let result = ctx.eval("x = 2 ^ 64 - 1").unwrap();
    assert!(result.var.is_some());

    let u64_max = Integer::from(u64::MAX);
    let var = ctx.var_get("x").unwrap();
    assert_eq!(&u64_max, var);
}

#[test]
fn last_eval() {
    let mut ctx = MathContext::new();

    let result = ctx.eval("x = 9 / 3").unwrap();
    assert!(result.var.is_some());

    let result = ctx.eval("_ * 4").unwrap();
    assert_eq!(result.value, Integer::from(12));
}

#[test]
fn underscore_disregard() {
    let mut ctx = MathContext::new();

    let result = ctx.eval("_ = 1 + 1").unwrap();
    assert!(result.var.is_none());
}
