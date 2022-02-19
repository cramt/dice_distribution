use std::{cmp, collections::HashMap, ops::Add};

use shoulda::Shoulda;

use crate::{BigInt, Int};

#[derive(Clone, Debug, Shoulda)]
pub struct PosibilitySpace(pub HashMap<Vec<Int>, BigInt>);

impl PosibilitySpace {
    pub fn multiply(self, rhs: BigInt) -> Self {
        let start = Self::empty();
        if rhs == 0 {
            return start;
        }
        (1..rhs).fold(start, |acc, _| acc + self.clone()) + self
    }

    pub fn empty() -> Self {
        PosibilitySpace(HashMap::new())
    }

    pub fn keep_highest(self, n: usize) -> Self {
        let capacity = self.0.capacity();
        PosibilitySpace(
            self.0
                .into_iter()
                .map(|(pos, amount)| (pos.into_iter().rev().take(n).rev().collect(), amount))
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

    pub fn keep_lowest(self, n: usize) -> Self {
        let capacity = self.0.capacity();
        Self(
            self.0
                .into_iter()
                .map(|(pos, amount)| (pos.into_iter().take(n).collect(), amount))
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

    pub fn count_successes(self, n: usize) -> Self {
        let capacity = self.0.capacity();
        Self(
            self.0
                .into_iter()
                .map(|(pos, amount)| {
                    (
                        vec![pos
                            .into_iter()
                            .map(|x| if x > n as Int { 1 } else { 0 })
                            .sum()],
                        amount,
                    )
                })
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

impl Add for PosibilitySpace {
    type Output = PosibilitySpace;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.is_empty() {
            return rhs;
        }
        let rhs = rhs.0;
        let mut new = HashMap::with_capacity(cmp::max(rhs.capacity(), self.0.capacity()));
        for (pos_x, amount_x) in self.0.into_iter() {
            for (pos_y, amount_y) in rhs.iter() {
                let mut pos = pos_x
                    .clone()
                    .into_iter()
                    .chain(pos_y.clone().into_iter())
                    .collect::<Vec<_>>();
                pos.sort_unstable();
                let amount = (amount_x * amount_y) + *new.get(&pos).unwrap_or(&0);
                new.insert(pos, amount);
            }
        }
        PosibilitySpace(new)
    }
}
