use std::{
    cmp,
    collections::HashMap,
    ops::{Add, Sub},
};

use shoulda::Shoulda;

use crate::{posibility_space::PosibilitySpace, BigInt, Int};

#[derive(Clone, Debug, Shoulda)]
pub struct Distribution(pub HashMap<Int, BigInt>);

impl From<PosibilitySpace> for Distribution {
    fn from(val: PosibilitySpace) -> Self {
        let capacity = val.0.capacity();
        Self(
            val.0
                .into_iter()
                .map(|(pos, amount)| (pos.into_iter().sum::<Int>(), amount))
                .fold(
                    HashMap::with_capacity(capacity),
                    |mut acc, (pos, amount)| {
                        let amount = amount + *acc.get(&pos).unwrap_or(&0);
                        acc.insert(pos, amount);
                        acc
                    },
                ),
        )
    }
}

impl Add for Distribution {
    type Output = Distribution;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.is_empty() {
            return rhs;
        }
        let rhs = rhs.0;
        let mut new = HashMap::with_capacity(cmp::max(rhs.capacity(), self.0.capacity()));
        for (pos_x, amount_x) in self.0.into_iter() {
            for (pos_y, amount_y) in rhs.iter() {
                let pos = pos_x + pos_y;
                let amount = (amount_x * amount_y) + *new.get(&pos).unwrap_or(&0);
                new.insert(pos, amount);
            }
        }
        Distribution(new)
    }
}

impl Sub for Distribution {
    type Output = Distribution;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.0.is_empty() {
            return rhs;
        }
        let rhs = rhs.0;
        let mut new = HashMap::with_capacity(cmp::max(rhs.capacity(), self.0.capacity()));
        for (pos_x, amount_x) in self.0.into_iter() {
            for (pos_y, amount_y) in rhs.iter() {
                let pos = pos_x - pos_y;
                let amount = (amount_x * amount_y) + *new.get(&pos).unwrap_or(&0);
                new.insert(pos, amount);
            }
        }
        Distribution(new)
    }
}

impl Distribution {
    pub fn mutate<T: Fn(Int) -> Int>(self, f: T) -> Self {
        Self(self.0.into_iter().map(|(l, r)| (f(l), r)).collect())
    }
}
