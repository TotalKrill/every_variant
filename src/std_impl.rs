use crate::*;

// ======================= Implementations ===================
//
// These are some kind of dumb implementations for "every variant" that can be basic structs
// There are infinite numbers of variants for vectors and the like, this will just give a small
// taste

macro_rules! std_impl {
    ($ty: ty, $($vals: expr),+) => {
        impl EveryVariant for $ty {
            fn every_variant() -> Vec<Self> {
                let vec = vec![
                $(
                    $vals,
                )+
                ];
                vec
            }
        }
    };
}

std_impl!((), ());

std_impl!(String, "example String".into());
std_impl!(&'static str, "&ŝtatic str!");

std_impl!(u8, 8);
std_impl!(u16, 16);
std_impl!(u32, 32);
std_impl!(u64, 64);
std_impl!(u128, 128);

std_impl!(i8, -8);
std_impl!(i16, -16);
std_impl!(i32, -32);
std_impl!(i64, -64);
std_impl!(i128, -128);

std_impl!(f32, 32., -32.);
std_impl!(f64, 32., -32.);

std_impl!(usize, 0, usize::MAX);
std_impl!(bool, true, false);

std_impl!(std::num::NonZeroU8, Self::new(8).unwrap());
std_impl!(std::num::NonZeroU16, Self::new(16).unwrap());
std_impl!(std::num::NonZeroU32, Self::new(32).unwrap());
std_impl!(std::num::NonZeroU64, Self::new(64).unwrap());
std_impl!(std::num::NonZeroU128, Self::new(128).unwrap());

std_impl!(
    std::num::NonZeroI8,
    Self::new(8).unwrap(),
    Self::new(-8).unwrap()
);
std_impl!(
    std::num::NonZeroI16,
    Self::new(16).unwrap(),
    Self::new(-16).unwrap()
);
std_impl!(
    std::num::NonZeroI32,
    Self::new(32).unwrap(),
    Self::new(-32).unwrap()
);
std_impl!(
    std::num::NonZeroI64,
    Self::new(64).unwrap(),
    Self::new(-64).unwrap()
);
std_impl!(
    std::num::NonZeroI128,
    Self::new(128).unwrap(),
    Self::new(-128).unwrap()
);

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

impl<T: EveryVariant + Clone + Sized> EveryVariant for Vec<T> {
    fn every_variant() -> Vec<Self> {
        // vector that contains one of every variant of the subsequent types
        let mut retvec = vec![T::every_variant()];

        // add an empty vector
        retvec.push(Vec::new());

        match T::every_variant().first().cloned() {
            Some(value) => {
                // add a single element vector
                let singlevec = vec![value.clone()];
                retvec.push(singlevec);

                // add a multi-element vector
                let mut largevec = Vec::new();
                while largevec.len() < 10 {
                    largevec.push(value.clone());
                }
                retvec.push(largevec);
            }
            None => {}
        };

        retvec
    }
}

#[cfg(feature = "ev_heapless")]
use heapless::{String as HString, Vec as HVec};

#[cfg(feature = "ev_heapless")]
impl<T, const N: usize> EveryVariant for HVec<T, N>
where
    T: EveryVariant + Clone + Sized,
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
impl<const N: usize> EveryVariant for HString<N> {
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
    use heapless::{String as HString, Vec as HVec};

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

        let opt_msg_len = Option::<MessageType>::every_variant().len();
        let text_len = String::every_variant().len();

        assert_eq!(opt_msg_len * text_len, all_diferent_messages.len());
    }

    #[cfg(feature = "ev_heapless")]
    #[test]
    fn heapless() {
        let _s = HString::<16>::every_variant();
        let _v = HVec::<u8, 16>::every_variant();
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
    pub struct TestUnnamed3(pub u16);

    #[test]
    fn messages_number() {
        let msgs = Message::every_variant().len();
        let messages_len = String::every_variant().len();
        let number_len = u32::every_variant().len();
        let opt_len = Option::<u64>::every_variant().len();
        let nest_len = Top::every_variant().len();
        let second_len = SecondTop::every_variant().len();

        assert_eq!(
            (messages_len * number_len * opt_len * nest_len * second_len),
            msgs
        );
    }

    #[test]
    fn opts_number() {
        let msgs = Option::<u64>::every_variant().len();
        assert_eq!(u64::every_variant().len() + 1, msgs);
    }

    #[derive(EveryVariant, Debug, Clone)]
    pub enum TestUnnamed1 {
        UnnamedSingle(u16),
        UnnamedMultiple1(u16, u32),
        UnnamedMultiple2(u16, u32, u64, i32),
    }

    #[test]
    fn unnamed1() {
        let msgs = TestUnnamed1::every_variant().len();
        let u16_len = u16::every_variant().len();
        let u32_len = u32::every_variant().len();
        let u64_len = u64::every_variant().len();
        let i32_len = i32::every_variant().len();
        assert_eq!(
            u16_len + u16_len * u32_len + u16_len * u32_len * u64_len * i32_len,
            msgs
        );
    }

    #[derive(EveryVariant, Debug, Clone)]
    pub struct TestUnnamed2(u16, u32, u64);

    #[test]
    fn unnamed2() {
        let msgs = TestUnnamed2::every_variant().len();
        let u16_len = u16::every_variant().len();
        let u32_len = u32::every_variant().len();
        let u64_len = u64::every_variant().len();
        assert_eq!(u16_len * u32_len * u64_len, msgs);
    }

    #[derive(EveryVariant, Debug, Clone)]
    pub struct Gen1<A: EveryVariant + Clone>(A);

    #[derive(EveryVariant, Debug, Clone)]
    pub struct Gen2<A: EveryVariant + Clone, B: EveryVariant + Clone>(A, B);

    #[derive(EveryVariant, Debug, Clone)]
    pub struct Generic1(Gen1<u8>, Gen2<u16, u32>);
    #[test]
    fn generic1() {
        let msgs = Generic1::every_variant().len();

        let gen1_len = Gen1::<u8>::every_variant().len();
        let gen2_len = Gen2::<u16, u32>::every_variant().len();

        assert_eq!(gen1_len * gen2_len, msgs);
    }

    #[derive(EveryVariant, Debug, Clone)]
    pub enum Generic2 {
        G1(Gen1<i8>),
        G2(Gen2<i16, i32>),
    }

    #[test]
    fn generic2() {
        let msgs = Generic2::every_variant().len();
        let gen1_len = Gen1::<i8>::every_variant().len();
        let gen2_len = Gen2::<i16, i32>::every_variant().len();
        assert_eq!(gen1_len + gen2_len, msgs);
    }

    #[derive(EveryVariant, Debug, Clone)]
    pub enum TestNamed1 {
        NamedSingle { first: u16 },
        NamedMultiple1 { first: u16, second: u32 },
        NamedMultiple2 { first: u16, second: u32, third: u64 },
    }

    #[test]
    fn named_enum() {
        let msgs = TestNamed1::every_variant().len();
        let u16_len = u16::every_variant().len();
        let u32_len = u32::every_variant().len();
        let u64_len = u64::every_variant().len();
        assert_eq!(
            u16_len + u16_len * u32_len + u16_len * u32_len * u64_len,
            msgs
        );
    }

    #[allow(unused)]
    #[derive(EveryVariant)]
    pub struct GenericDerive<T> {
        value: T,
    }

    #[allow(unused)]
    #[derive(EveryVariant)]
    pub enum GenericEnum<T> {
        One(T),
        Two(u32),
    }

    #[allow(unused)]
    #[derive(EveryVariant)]
    pub struct MultiGeneric<A, B>(A, B);

    #[test]
    fn generic_everyvariant() {
        let msgs = GenericDerive::<u32>::every_variant().len();
        assert_eq!(u32::every_variant().len(), msgs);

        let msgs = GenericEnum::<u32>::every_variant().len();
        assert_eq!(2 * u32::every_variant().len(), msgs);

        let msgs = MultiGeneric::<u32, u32>::every_variant().len();
        assert_eq!(
            u32::every_variant().len() * u32::every_variant().len(),
            msgs
        );
    }
}
