#![cfg_attr(feature = "nightly",
    cfg_attr(all(), doc = include_str!("../README.md")),
)]

extern crate macro_rules_attribute_proc_macro;

pub use macro_rules_attribute_proc_macro::{
    macro_rules_attribute,
    macro_rules_derive,
};

#[doc(no_inline)]
pub use macro_rules_attribute as apply;
