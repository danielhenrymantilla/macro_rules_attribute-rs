# `::macro_rules_attribute`

Use declarative macros in attribute or derive position.

```rust ,ignore
macro_rules! my_fancy_decorator { /* … */ }

#[apply(my_fancy_decorator!)]
struct Foo { /* … */ }
```

```rust ,ignore
macro_rules! MyFancyDerive { /* … */ }

#[derive(MyFancyDerive!)]
struct Foo { /* … */ }
```

[![Latest version](https://img.shields.io/crates/v/macro_rules_attribute.svg)](https://crates.io/crates/macro_rules_attribute)
[![Documentation](https://docs.rs/macro_rules_attribute/badge.svg)](https://docs.rs/macro_rules_attribute)
![License](https://img.shields.io/crates/l/macro_rules_attribute.svg)

## Motivation

`macro_rules!` macros can be extremely powerful, but their call-site ergonomics
are sometimes not great, especially when decorating item definitions.

Indeed, compare:

```rust ,ignore
foo! {
    struct Struct {
        some_field: SomeType,
    }
}
```

to:

```rust ,ignore
#[foo]
struct Struct {
    some_field: SomeType,
}
```

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

With this crate's <code>#\[[apply]\]</code> and <code>#\[[derive]\]</code>
attributes, it is now possible to use `proc_macro_attribute` syntax to apply a
`macro_rules!` macro:

[apply]: https://docs.rs/macro_rules_attribute/0.1.0-rc1/macro_rules_attribute/attr.apply.html
[derive]: https://docs.rs/macro_rules_attribute/0.1.0-rc1/macro_rules_attribute/attr.derive.html

```rust
#[macro_use]
extern crate macro_rules_attribute;

macro_rules! foo {
    // …
    # ( $($tt:tt)* ) => ()
}

macro_rules! Bar {
    // …
    # ( $($tt:tt)* ) => ()
}

#[apply(foo)]
#[derive(Debug, Bar!)]
struct Struct {
    some_field: SomeType,
}
#
# fn main() {}
```

without even depending on [`::quote`], [`::syn`] or [`::proc-macro2`], for
**fast compile times**.

  - Note: for even faster compile times, feel free to disable the `derive-alias`
    Cargo feature, should you not use it.

    On my machine, that feature requires around 0.3s of extra compile-time,
    which is not much, but still a 25% increase w.r.t. `--no-default-features`.

[`macro_rules_attribute`]: https://docs.rs/macro_rules_attribute_proc_macro/0.0.1/macro_rules_attribute_proc_macro/attr.macro_rules_attribute.html
[`macro_rules_derive`]: https://docs.rs/macro_rules_attribute_proc_macro/0.0.1/macro_rules_attribute_proc_macro/attr.macro_rules_derive.html
[`::paste`]: https://docs.rs/paste
[`::proc-macro2`]: https://docs.rs/proc_macro2
[`::syn`]: https://docs.rs/syn
[`::quote`]: https://docs.rs/quote

# Example

Deriving getters for a (non-generic) `struct`:

```rust
#[macro_use]
extern crate macro_rules_attribute;

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
    #[apply(make_getters)]
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

# fn main() {}
```

# Debugging

An optional compilation feature, `"verbose-expansions"` can be used to print at
compile-time the exact output of each macro invocation from this crate:

```toml
[dependencies]
macro_rules_attribute.version = "..."
macro_rules_attribute.features = ["verbose-expansions"]
```

# Bonus tricks

### `derive` aliases

```rust
# fn main() {}
#[macro_use]
extern crate macro_rules_attribute;

derive_alias! {
    #[derive(Ord!)] = #[derive(PartialEq, Eq, PartialOrd, Ord)];
}

#[derive(Debug, Clone, Copy, Ord!)]
struct Foo {
    // …
}
```

  - See [`derive_alias!`] and <code>#\[[derive]\]</code> for more info.

[`derive_alias!`]: https://docs.rs/macro_rules_attribute/0.1.0-rc1/macro_rules_attribute/macro.derive_alias.html

### `cfg` aliases

```rust
# fn main() {}
#[macro_use]
extern crate macro_rules_attribute;

attribute_alias! {
    #[apply(complex_cfg!)] = #[cfg(
        any(
            any(
                foo,
                feature = "bar",
            ),
            all(
                target_os = "fenestrations",
                not(target_arch = "Pear"),
            ),
        ),
    )];
}

#[apply(complex_cfg!)]
mod some_item { /* … */ }
```
