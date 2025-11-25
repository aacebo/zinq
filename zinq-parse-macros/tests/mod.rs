use zinq_parse_macros::{zinq_attribute, zinq_derive};

#[zinq_derive(Output=TokenStream)]
pub struct DeriveElement;

#[zinq_attribute]
pub struct AttributeElement;
