fn main() {
    let mut context = jnk::MathContext::new();
    context.eval("x = 1 + (2 * 4) / 3").unwrap();
}
