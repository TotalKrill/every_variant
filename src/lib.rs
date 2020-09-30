pub use every_variant::*;

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

impl EveryVariant for bool {
    fn every_variant() -> Vec<Self> {
        let vec = vec![true, false];
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

#[cfg(feature = "ev_heapless")]
use heapless::{ArrayLength, String as HString, Vec as HVec};

#[cfg(feature = "ev_heapless")]
impl<T, N> EveryVariant for HVec<T, N>
where
    T: EveryVariant + Clone + Sized,
    N: ArrayLength<T>,
{
    fn every_variant() -> Vec<Self> {
        let mut vec = HVec::new();

        for v in T::every_variant() {
            vec.push(v).ok();
        }

        vec![vec]
    }
}

#[cfg(feature = "ev_heapless")]
impl<N> EveryVariant for HString<N>
where
    N: ArrayLength<u8>,
{
    fn every_variant() -> Vec<Self> {
        let mut s = HString::new();
        s.push_str("hello").ok();
        vec![s]
    }
}

#[cfg(test)]
mod tests {
    use crate::EveryVariant;
    #[cfg(feature = "ev_heapless")]
    use heapless::{consts::U16, String as HString, Vec as HVec};

    /// Type of the message
    #[derive(EveryVariant, Debug, Clone)]
    enum MessageType {
        Codified,
        Markdown,
        Html,
    }

    /// This type can come in  4 different variants due the option
    #[derive(EveryVariant, Debug, Clone)]
    struct FormattedMessage {
        /// Enum dictating how to render the string, None means its hidden
        rendermethod: Option<MessageType>,
        /// The optional content of the message
        text: String,
    }

    #[test]
    fn small_example() {
        let all_diferent_messages = FormattedMessage::every_variant();
        println!("{:#?}", all_diferent_messages);
        assert_eq!(4, all_diferent_messages.len());
    }

    #[cfg(feature = "ev_heapless")]
    #[test]
    fn heapless() {
        let _s = HString::<U16>::every_variant();
        let _v = HVec::<u8, U16>::every_variant();
    }

    #[derive(EveryVariant, Debug, Clone)]
    pub struct Message {
        pub message: String,
        pub number: u32,
        pub opt: Option<u64>,
        pub nest: Top,
        pub second: SecondTop,
    }

    #[derive(EveryVariant, Debug, Clone)]
    pub enum SecondTop {
        One,
        Two(Nested),
        Three,
    }

    #[derive(EveryVariant, Debug, Clone)]
    pub enum Top {
        One,
        Nested(Nested),
    }

    #[derive(EveryVariant, Debug, Clone)]
    pub enum Nested {
        First,
        Second,
        Third,
    }

    #[derive(EveryVariant, Debug, Clone)]
    pub enum TestUnnamed1 {
        UnnamedSingle(u16),
        UnnamedMultiple1(u16, u32),
        UnnamedMultiple2(u16, u32, u64),
    }

    #[derive(EveryVariant, Debug, Clone)]
    pub struct TestUnnamed2(u16, u32, u64);

    // #[derive(EveryVariant, Debug, Clone)]
    // pub struct Gen<A: EveryVariant + Clone>(A);

    // #[derive(EveryVariant, Debug, Clone)]
    // pub struct Generic(Gen<u8>);

    #[derive(EveryVariant, Debug, Clone)]
    pub struct TestUnnamed3(pub u16);

    #[test]
    fn messages_number() {
        let msgs = Message::every_variant().len();
        assert_eq!(40, msgs);
    }

    #[test]
    fn opts_number() {
        let msgs = Option::<u64>::every_variant().len();
        assert_eq!(2, msgs);
    }

    #[test]
    fn unnamed1() {
        let msgs = TestUnnamed1::every_variant().len();
        assert_eq!(3, msgs);
    }

    #[test]
    fn unnamed2() {
        let msgs = TestUnnamed2::every_variant().len();
        assert_eq!(1, msgs);
    }
}
