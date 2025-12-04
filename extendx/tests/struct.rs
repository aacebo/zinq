use extendx::Extend;

#[derive(Default, Extend)]
#[allow(unused)]
struct A {
    pub a: String,
}

#[derive(Default, Extend)]
#[allow(unused)]
struct B {
    #[proxy]
    inner_a: A,
    pub b: usize,
}

#[derive(Default, Extend)]
struct C {
    #[proxy]
    inner_b: B,
    pub c: bool,
}

#[test]
pub fn should_extend() {
    let c = C::default();

    debug_assert_eq!(c.b(), &0);
    debug_assert_eq!(c.c, false);
}
