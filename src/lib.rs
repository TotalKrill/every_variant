pub use every_variant::*;

pub trait EveryVariant: Sized {
    fn every_variant() -> Vec<Self>;
}

// ======================= Implementations ===================
impl EveryVariant for String {
    fn every_variant() -> Vec<Self> {
        vec!["example".into()]
    }
}
impl EveryVariant for u32 {
    fn every_variant() -> Vec<Self> {
        vec![32]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
