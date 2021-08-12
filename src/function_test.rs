use crate::*;

use std::str::FromStr;

// plans to help add crashtesting are just examined in this file, please ignore


#[allow(unused)]
#[crashtest_every_variant] // <- this should create the function crashtest_parse_double_strings_into_numbers() -> ()
fn parse_double_strings_into_numbers(s1: String, s2: String) -> (u16, u16) {
    let s1: u16 = u16::from_str(&s1).unwrap_or_default();
    let s2: u16 = u16::from_str(&s2).unwrap_or_default();

    (s1, s2)
}

#[allow(unused)]
#[crashtest_every_variant] // <- this should create the function crashtest_parse_single_string() -> ()
fn parse_single_string(s1: String) -> u16 {
    let s1: u16 = u16::from_str(&s1).unwrap_or_default();

    s1
}

#[cfg(test)]
mod tests {
    pub use super::*;

    // this should just use the created function, and create tests for them all
    craschtests!{
        parse_double_strings_into_numbers,
        parse_single_string,
    };

    // like this ->
    #[test]
    fn crashtest_parse_double_strings_into_numbers() {
        for s1 in String::every_variant() {
            for s2 in String::every_variant() {
                let _ = parse_double_strings_into_numbers(s1.clone(), s2.clone());
            }
        }
    }

    #[test]
    fn crashtest_parse_single_string() {
        for s1 in String::every_variant() {
            let _ = parse_single_string(s1.clone());
        }
    }
}
