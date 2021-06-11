# `::macro_rules_attribute`

Use declarative macros as proc_macro attributes or derives.

[![Latest version](https://img.shields.io/crates/v/macro_rules_attribute.svg)](https://crates.io/crates/macro_rules_attribute)
[![Documentation](https://docs.rs/macro_rules_attribute/badge.svg)](https://docs.rs/macro_rules_attribute)
![License](https://img.shields.io/crates/l/macro_rules_attribute.svg)

## Motivation

`macro_rules!` macros can be extremely powerful, but their call-site ergonomics
are sometimes not great, especially when decorating item definitions.

Indeed, compare:

 1. ```rust
    # #[cfg(any())]
    foo! {
        struct Struct {
            some_field: SomeType,
        }
    }
    ```

 1. ```rust
    # #[cfg(any())]
    #[foo]
    struct Struct {
        some_field: SomeType,
    }
    ```
___

 1. The former does not scale well, since it leads to **rightward drift and
"excessive" braces**.

 1. But on the other hand, the latter requires setting up a dedicated crate for
    the compiler, a `proc-macro` crate. And 99% of the time this will pull the
    [`::syn`] and [`::quote`] dependencies, which have **a
    non-negligible compile-time overhead** (the first time they are compiled).

     - note: these crates are a wonderful piece of technology, and can lead to
       extremely powerful macros. When the logic of the macro is so complicated
       that it requires a recursive `tt` muncher when implemented as a
       `macro_rules!` macro, it is definitely time to be using a `proc`edural
       macro.

       Anything involving `ident` generation / derivation, for instance, will very
       often require `proc`edural macros, unless it is simple enough for
       [`::paste`] to handle it.

___

## Solution

With the [`macro_rules_attribute`] and [`macro_rules_derive`] attributes, it is
now possible to use `proc_macro_attribute` syntax to apply a `macro_rules!`
macro:

```rust
# use ::macro_rules_attribute::macro_rules_attribute;
# macro_rules! foo {($($tt:tt)*) => ()}
#
#[macro_rules_attribute(foo!)]
struct Struct {
    some_field: SomeType,
}
```

without even depending on [`::quote`], [`::syn`] or [`::proc-macro2`], for
**fast compile times**.

[`macro_rules_attribute`]: https://docs.rs/macro_rules_attribute_proc_macro/0.0.1/macro_rules_attribute_proc_macro/attr.macro_rules_attribute.html
[`macro_rules_derive`]: https://docs.rs/macro_rules_attribute_proc_macro/0.0.1/macro_rules_attribute_proc_macro/attr.macro_rules_derive.html
[`::paste`]: https://docs.rs/paste
[`::proc-macro2`]: https://docs.rs/proc_macro2
[`::syn`]: https://docs.rs/syn
[`::quote`]: https://docs.rs/quote

# Example

Deriving getters for a (non-generic) `struct`:

```rust
# macro_rules! ignore {($($tt:tt)*) => () }
# ignore! {
#[macro_use]
extern crate macro_rules_attribute;
# }

macro_rules! make_getters {(
    $(#[$struct_meta:meta])*
    $struct_vis:vis
    struct $StructName:ident {
        $(
            $(#[$field_meta:meta])*
            $field_vis:vis // this visibility will be applied to the getters instead
            $field_name:ident : $field_ty:ty
        ),* $(,)?
    }
) => (
    // First, generate the struct definition we have been given, but with
    // private fields instead.
    $(#[$struct_meta])*
    $struct_vis
    struct $StructName {
        $(
            $(#[$field_meta])*
            // notice the lack of visibility => private fields
            $field_name: $field_ty,
        )*
    }

    // Then, implement the getters:
    impl $StructName {
        $(
            #[inline]
            $field_vis
            fn $field_name (self: &'_ Self)
                -> &'_ $field_ty
            {
                &self.$field_name
            }
        )*
    }
)}

mod example {
# use ::macro_rules_attribute::macro_rules_attribute;
    #[macro_rules_attribute(make_getters!)]
    /// The macro handles meta attributes such as docstrings
    pub
    struct Person {
        pub
        name: String,

        pub
        age: u8,
    }
}
use example::Person;

fn is_new_born (person: &'_ Person)
    -> bool
{
    // person.age == 0
    // ^ error[E0616]: field `age` of struct `example::Person` is private
    *person.age() == 0
}
```

# Debugging

An optional compilation feature, `"verbose-expansions"` can be used to print at
compile_time the exact macro call:

```toml
[dependencies]
macro_rules_attribute = { version = "...", features = ["verbose-expansions"] }
```

# The `#[apply(macro!)]` shorthand

Just a convenient shorthand for `#[macro_rules_attribute(macro!)]`:

### Example

```rust,ignore
#[macro_use]
extern crate macro_rules_attribute;

macro_rules! complex_cfg {( $item:item ) => (
    #[cfg(any(
        any(
            foo,
            feature = "bar",
        ),
        all(
            target_os = "fenestrations",
            not(target_arch = "Pear"),
        ),
    ))]
    $item
)}

#[apply(complex_cfg!)]
mod some_item { /* … */ }
```
