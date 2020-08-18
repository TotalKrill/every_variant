pub use every_variant::*;

/// This requires that each underlying structure or nest has this implemented
pub trait EveryVariant: Sized {
    /// A vector of variants that should contain every possible variant of the struct or enum
    fn every_variant() -> Vec<Self>;
}

// ======================= Implementations ===================
//
// These are some kind of dumb implementations for "every variant" that can be basic structs
// There are infinite numbers of variants for "Scalars"

impl EveryVariant for String {
    fn every_variant() -> Vec<Self> {
        let vec = vec!["example String".into()];
        vec
    }
}
impl EveryVariant for &'static str {
    fn every_variant() -> Vec<Self> {
        let vec = vec!["&static str!"];
        vec
    }
}

impl EveryVariant for u32 {
    fn every_variant() -> Vec<Self> {
        let vec = vec![32];
        vec
    }
}

impl EveryVariant for u64 {
    fn every_variant() -> Vec<Self> {
        let vec = vec![64];
        vec
    }
}
impl EveryVariant for u16 {
    fn every_variant() -> Vec<Self> {
        let vec = vec![16];
        vec
    }
}
impl EveryVariant for u8 {
    fn every_variant() -> Vec<Self> {
        let vec = vec![8];
        vec
    }
}

impl EveryVariant for i32 {
    fn every_variant() -> Vec<Self> {
        let vec = vec![-32];
        vec
    }
}

impl EveryVariant for i64 {
    fn every_variant() -> Vec<Self> {
        let vec = vec![-64];
        vec
    }
}
impl EveryVariant for i16 {
    fn every_variant() -> Vec<Self> {
        let vec = vec![-16];
        vec
    }
}
impl EveryVariant for i8 {
    fn every_variant() -> Vec<Self> {
        let vec = vec![-8];
        vec
    }
}

impl EveryVariant for usize {
    fn every_variant() -> Vec<Self> {
        let vec = vec![100];
        vec
    }
}

impl EveryVariant for f32 {
    fn every_variant() -> Vec<Self> {
        let vec = vec![16.32];
        vec
    }
}
impl EveryVariant for f64 {
    fn every_variant() -> Vec<Self> {
        let vec = vec![16.64];
        vec
    }
}

impl EveryVariant for std::num::NonZeroU8 {
    fn every_variant() -> Vec<Self> {
        let vec = vec![Self::new(4).unwrap()];
        vec
    }
}

impl<T: EveryVariant + Clone + Sized> EveryVariant for Option<T> {
    fn every_variant() -> Vec<Self> {
        let mut vec = Vec::new();
        vec.push(None);

        let tvec = T::every_variant();
        let mut tvec = tvec.into_iter().map(|t| Some(t)).collect();
        vec.append(&mut tvec);

        vec
    }
}

impl EveryVariant for () {
    fn every_variant() -> Vec<Self> {
        let mut vec = Vec::new();
        vec.push(());
        vec
    }
}

impl<T: EveryVariant + Clone + Sized, E: EveryVariant + Clone + Sized> EveryVariant
    for Result<T, E>
{
    fn every_variant() -> Vec<Self> {
        let mut vec = Vec::new();

        let tvec = T::every_variant();
        let mut tvec = tvec.into_iter().map(|t| Ok(t)).collect();
        vec.append(&mut tvec);

        let evec = E::every_variant();
        let mut evec = evec.into_iter().map(|t| Err(t)).collect();
        vec.append(&mut evec);

        vec
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
