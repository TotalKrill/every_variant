use all_variants::*;

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

fn main() {
    for var in Message::every_variant() {
        println!("{:?}", var);
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
}
