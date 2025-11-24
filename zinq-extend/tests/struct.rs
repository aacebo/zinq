use zinq_extend::extend;

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
    let _c = C {
        a: String::new(),
        b: 1,
        c: true,
    };
}
