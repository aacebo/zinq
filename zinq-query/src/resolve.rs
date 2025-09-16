pub trait Resolve<P, T>: zinq_schema::Schema {
    fn resolve(&self, ctx: &crate::Context<P, T>) -> crate::Result<T>;
}

pub trait FieldResolve<P, T>: zinq_schema::Schema {
    fn resolve(&self, ctx: &crate::FieldContext<P, T>) -> crate::Result<T>;
}
