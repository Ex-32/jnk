fn main() {
    let mut context = jnk::context::MathContext::new();
    context.eval("x = ((4 - (3 * 2)) * 8 / -4) ^ -2").unwrap();
}
