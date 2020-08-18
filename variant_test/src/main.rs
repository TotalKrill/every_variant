use every_variant::*;

pub trait EveryVariant: Sized {
    fn every_variant() -> Vec<Self>;
}

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

#[derive(EveryVariant, Debug, Clone)]
pub struct Message {
    pub message: String,
    pub number: u32,
    pub nest: Top,
    pub second: SecondTop,
}

// impl EveryVariant for Message {
//     fn every_variant() -> Vec<Self> {
//         let mut vec = Vec::new();

//         let msgs = String::every_variant();
//         let numbers = u32::every_variant();
//         let nests = Top::every_variant();
//         let seconds = SecondTop::every_variant();

//         for message in &msgs {
//             for number in &numbers {
//                 for nest in &nests {
//                     for second in &seconds {
//                         let var = Self {
//                             message: message.clone(),
//                             number: number.clone(),
//                             nest: nest.clone(),
//                             second: second.clone(),
//                         };
//                         vec.push(var);
//                     }
//                 }
//             }
//         }

//         vec
//     }
// }

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
        assert_eq!(20, msgs);
    }
}
