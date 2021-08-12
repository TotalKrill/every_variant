pub use every_variant_macro::*;

/// Trait that supplies a function to generate a vector containing all possible variants in a tree
pub trait EveryVariant: Sized {
    /// A vector of variants that should contain every possible variant of the struct or enum
    fn every_variant() -> Vec<Self>;
    /// Helper function, loops of each variant in an enum. Else does nothing.
    fn for_every_variant<F: Fn(&Self)>(closure: F) {
        // Do nothing
        let _ = closure;
    }
}

pub mod std_impl;
pub use std_impl::*;
