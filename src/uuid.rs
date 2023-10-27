use uuid::Uuid;

use crate::EveryVariant;

impl EveryVariant for Uuid {
    fn every_variant() -> Vec<Self> {
        vec![Uuid::default()]
    }
}
