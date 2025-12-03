use extendx::extend;

#[extend]
#[allow(unused)]
struct A {
    pub a: String,
}

#[extend(A)]
#[allow(unused)]
struct B {
    pub b: usize,
}

#[extend(B)]
struct C {
    pub c: bool,
}

#[test]
pub fn should_extend() {
    let c = C {
        a: String::new(),
        b: 1,
        c: true,
    };

    debug_assert_eq!(c.a, "");
    debug_assert_eq!(c.b, 1);
    debug_assert_eq!(c.c, true);
}
