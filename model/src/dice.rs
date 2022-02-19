use crate::{Int, posibility_space::PosibilitySpace};

#[derive(Clone, Debug)]
pub struct Dice(pub Int);

impl From<Dice> for PosibilitySpace {
    fn from(val: Dice) -> Self {
        PosibilitySpace((1..=val.0).into_iter().map(|x| (vec![x], 1)).collect())
    }
}