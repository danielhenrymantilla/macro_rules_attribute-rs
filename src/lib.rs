#![cfg_attr(feature = "nightly",
    feature(external_doc),
    doc(include = "../README.md"),
)]

#[cfg(any())]
mod proc_macro;

pub use proc_macro::{
    macro_rules_attribute,
    macro_rules_derive,
};
