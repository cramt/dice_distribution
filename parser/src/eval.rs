use std::{
    error::Error,
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use model::{dice::Dice, distribution::Distribution, posibility_space::PosibilitySpace, Int};
use shoulda::Shoulda;

use crate::ast::{Operator, Value};

#[derive(Debug, Shoulda)]
pub enum EvalValue {
    Constant(i32),
    PreDice(PosibilitySpace),
    PostDice(Distribution),
}

impl From<EvalValue> for Distribution {
    fn from(val: EvalValue) -> Self {
        match val {
            EvalValue::Constant(c) => Distribution([(c as Int, 1)].into()),
            EvalValue::PreDice(pre) => pre.into(),
            EvalValue::PostDice(post) => post,
        }
    }
}

#[derive(Debug, Shoulda)]
pub enum EvalError {
    InvalidArgForDice,
    InvalidArgForKeepHeighest,
    InvalidArgForCountSuccesses,
    MultiplyDiceWithDice,
    DivideDiceWithDice,
    DivideByZero,
}

impl Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::InvalidArgForDice => write!(f, "Eval Error: Invalid Arg for Dice"),
            EvalError::MultiplyDiceWithDice => {
                write!(f, "Eval Error: Tried multiplying dice with other dice")
            }
            EvalError::DivideDiceWithDice => {
                write!(f, "Eval Error: Tried dividing dice with other dice")
            }
            EvalError::DivideByZero => {
                write!(f, "Eval Error: Tried dividing by zero")
            }
            EvalError::InvalidArgForKeepHeighest => {
                write!(f, "Eval Error: Invalid arg for keep heighest")
            }
            EvalError::InvalidArgForCountSuccesses => {
                write!(f, "Eval Error: Invalid arg for count successes")
            }
        }
    }
}

impl Error for EvalError {}

impl Operator {
    pub fn eval(self) -> Result<EvalValue, EvalError> {
        match self {
            Operator::Plus(l, r) => l + r,
            Operator::Minus(l, r) => l - r,
            Operator::Multiply(l, r) => l * r,
            Operator::Divide(l, r) => l / r,
            Operator::KeepHighest(l, r) => l.keep_heighest(r),
            Operator::CountSuccesses(l, r) => todo!(),
            Operator::Dice(l, r) => l.dice(r),
        }
    }
}

impl Value {
    pub fn eval(self) -> Result<EvalValue, EvalError> {
        match self {
            Value::Default => Ok(EvalValue::Constant(0)),
            Value::Constant(c) => Ok(EvalValue::Constant(c)),
            Value::Operator(o) => o.as_ref().clone().eval(),
        }
    }

    pub fn count_successes(self, rhs: Self) -> Result<EvalValue, EvalError> {
        match (self, rhs) {
            (Value::Default, Value::Default) => Ok(EvalValue::PreDice(
                PosibilitySpace::from(Dice(10)).count_successes(6),
            )),
            (Value::Default, Value::Constant(c)) => {
                if c < 1 {
                    Err(EvalError::InvalidArgForCountSuccesses)
                } else {
                    Ok(EvalValue::PreDice(
                        PosibilitySpace::from(Dice(10)).count_successes(c as usize),
                    ))
                }
            }
            (Value::Default, Value::Operator(o)) => match o.as_ref().clone().eval()? {
                EvalValue::Constant(c) => {
                    if c < 1 {
                        Err(EvalError::InvalidArgForCountSuccesses)
                    } else {
                        Ok(EvalValue::PreDice(
                            PosibilitySpace::from(Dice(10)).count_successes(c as usize),
                        ))
                    }
                }
                _ => Err(EvalError::InvalidArgForCountSuccesses),
            },
            (Value::Operator(o), Value::Default) => match o.as_ref().clone().eval()? {
                EvalValue::PreDice(d) => Ok(EvalValue::PreDice(d.count_successes(6))),
                _ => Err(EvalError::InvalidArgForCountSuccesses),
            },
            (Value::Operator(o), Value::Constant(c)) => {
                if c < 1 {
                    Err(EvalError::InvalidArgForCountSuccesses)
                } else {
                    match o.as_ref().clone().eval()? {
                        EvalValue::PreDice(d) => {
                            Ok(EvalValue::PreDice(d.count_successes(c as usize)))
                        }
                        _ => Err(EvalError::InvalidArgForCountSuccesses),
                    }
                }
            }
            (Value::Operator(l), Value::Operator(r)) => {
                match (l.as_ref().clone().eval()?, r.as_ref().clone().eval()?) {
                    (EvalValue::PreDice(d), EvalValue::Constant(c)) => {
                        if c < 1 {
                            Err(EvalError::InvalidArgForCountSuccesses)
                        } else {
                            Ok(EvalValue::PreDice(d.count_successes(c as usize)))
                        }
                    }
                    _ => Err(EvalError::InvalidArgForCountSuccesses),
                }
            }
            _ => Err(EvalError::InvalidArgForCountSuccesses),
        }
    }

    pub fn keep_heighest(self, rhs: Self) -> Result<EvalValue, EvalError> {
        match (self, rhs) {
            (Value::Default, Value::Default) => Ok(EvalValue::PreDice(
                PosibilitySpace::from(Dice(20)).multiply(2).keep_highest(1),
            )),
            (Value::Default, Value::Constant(c)) => {
                if c < 1 {
                    Err(EvalError::InvalidArgForKeepHeighest)
                } else {
                    Ok(EvalValue::PreDice(
                        PosibilitySpace::from(Dice(20))
                            .multiply(2)
                            .keep_highest(c as usize),
                    ))
                }
            }
            (Value::Default, Value::Operator(o)) => match o.as_ref().clone().eval()? {
                EvalValue::Constant(c) => {
                    if c < 1 {
                        Err(EvalError::InvalidArgForKeepHeighest)
                    } else {
                        Ok(EvalValue::PreDice(
                            PosibilitySpace::from(Dice(20))
                                .multiply(2)
                                .keep_highest(c as usize),
                        ))
                    }
                }
                _ => Err(EvalError::InvalidArgForKeepHeighest),
            },
            (Value::Operator(o), Value::Default) => match o.as_ref().clone().eval()? {
                EvalValue::PreDice(d) => Ok(EvalValue::PreDice(d.keep_highest(1))),
                _ => Err(EvalError::InvalidArgForKeepHeighest),
            },
            (Value::Operator(o), Value::Constant(c)) => {
                if c < 1 {
                    Err(EvalError::InvalidArgForKeepHeighest)
                } else {
                    match o.as_ref().clone().eval()? {
                        EvalValue::PreDice(d) => Ok(EvalValue::PreDice(d.keep_highest(c as usize))),
                        _ => Err(EvalError::InvalidArgForKeepHeighest),
                    }
                }
            }
            (Value::Operator(l), Value::Operator(r)) => {
                match (l.as_ref().clone().eval()?, r.as_ref().clone().eval()?) {
                    (EvalValue::PreDice(d), EvalValue::Constant(c)) => {
                        if c < 1 {
                            Err(EvalError::InvalidArgForKeepHeighest)
                        } else {
                            Ok(EvalValue::PreDice(d.keep_highest(c as usize)))
                        }
                    }
                    _ => Err(EvalError::InvalidArgForKeepHeighest),
                }
            }
            _ => Err(EvalError::InvalidArgForKeepHeighest),
        }
    }

    pub fn dice(self, rhs: Self) -> Result<EvalValue, EvalError> {
        match (self, rhs) {
            (Value::Default, Value::Default) => Ok(EvalValue::PreDice(Dice(6).into())),
            (Value::Default, Value::Constant(c)) => {
                if c < 1 {
                    Err(EvalError::InvalidArgForDice)
                } else {
                    Ok(EvalValue::PreDice(Dice(c as Int).into()))
                }
            }
            (Value::Default, Value::Operator(o)) => match o.as_ref().clone().eval()? {
                EvalValue::Constant(c) => {
                    if c < 1 {
                        Err(EvalError::InvalidArgForDice)
                    } else {
                        Ok(EvalValue::PreDice(Dice(c as Int).into()))
                    }
                }
                _ => Err(EvalError::InvalidArgForDice),
            },
            (Value::Constant(c), Value::Default) => {
                if c < 1 {
                    Err(EvalError::InvalidArgForDice)
                } else {
                    Ok(EvalValue::PreDice(
                        PosibilitySpace::from(Dice(6)).multiply(c as u32),
                    ))
                }
            }
            (Value::Constant(l), Value::Constant(r)) => {
                if l < 1 || r < 1 {
                    Err(EvalError::InvalidArgForDice)
                } else {
                    Ok(EvalValue::PreDice(
                        PosibilitySpace::from(Dice(r as Int)).multiply(l as u32),
                    ))
                }
            }
            (Value::Constant(c), Value::Operator(o)) => match o.as_ref().clone().eval()? {
                EvalValue::Constant(x) => {
                    if c < 1 || x < 1 {
                        Err(EvalError::InvalidArgForDice)
                    } else {
                        Ok(EvalValue::PreDice(
                            PosibilitySpace::from(Dice(x as Int)).multiply(c as u32),
                        ))
                    }
                }
                _ => Err(EvalError::InvalidArgForDice),
            },

            (Value::Operator(o), Value::Default) => match o.as_ref().clone().eval()? {
                EvalValue::Constant(c) => {
                    if c < 1 {
                        Err(EvalError::InvalidArgForDice)
                    } else {
                        Ok(EvalValue::PreDice(
                            PosibilitySpace::from(Dice(6)).multiply(c as u32),
                        ))
                    }
                }
                _ => Err(EvalError::InvalidArgForDice),
            },
            (Value::Operator(o), Value::Constant(c)) => match o.as_ref().clone().eval()? {
                EvalValue::Constant(x) => {
                    if x < 1 || c < 1 {
                        Err(EvalError::InvalidArgForDice)
                    } else {
                        Ok(EvalValue::PreDice(
                            PosibilitySpace::from(Dice(c as Int)).multiply(x as u32),
                        ))
                    }
                }
                _ => Err(EvalError::InvalidArgForDice),
            },
            (Value::Operator(l), Value::Operator(r)) => {
                match (l.as_ref().clone().eval()?, r.as_ref().clone().eval()?) {
                    (EvalValue::Constant(l), EvalValue::Constant(r)) => {
                        if l < 1 || r < 1 {
                            Err(EvalError::InvalidArgForDice)
                        } else {
                            Ok(EvalValue::PreDice(
                                PosibilitySpace::from(Dice(r as Int)).multiply(l as u32),
                            ))
                        }
                    }
                    _ => Err(EvalError::InvalidArgForDice),
                }
            }
        }
    }
}

impl Div for Value {
    type Output = Result<EvalValue, EvalError>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Default, Value::Default) => Ok(EvalValue::Constant(1)),
            (Value::Default, Value::Constant(c)) => {
                if c == 0 {
                    Err(EvalError::DivideByZero)
                } else {
                    Ok(EvalValue::Constant(1 / c))
                }
            }
            (Value::Default, Value::Operator(o)) => match o.as_ref().clone().eval()? {
                EvalValue::Constant(c) => {
                    if c == 0 {
                        Ok(EvalValue::Constant(1 / c))
                    } else {
                        Err(EvalError::DivideByZero)
                    }
                }
                EvalValue::PreDice(d) => {
                    let d: Distribution = d.into();
                    if d.0.iter().any(|(x, _)| *x == 0) {
                        Err(EvalError::DivideByZero)
                    } else {
                        Ok(EvalValue::PostDice(d.mutate(|x| 1 / x)))
                    }
                }
                EvalValue::PostDice(d) => {
                    if d.0.iter().any(|(x, _)| *x == 0) {
                        Err(EvalError::DivideByZero)
                    } else {
                        Ok(EvalValue::PostDice(d.mutate(|x| 1 / x)))
                    }
                }
            },
            (Value::Constant(c), Value::Default) => Ok(EvalValue::Constant(c)),
            (Value::Constant(l), Value::Constant(r)) => {
                if r == 0 {
                    Err(EvalError::DivideByZero)
                } else {
                    Ok(EvalValue::Constant(l / r))
                }
            }
            (Value::Constant(c), Value::Operator(o)) => match o.as_ref().clone().eval()? {
                EvalValue::Constant(x) => {
                    if x == 0 {
                        Err(EvalError::DivideByZero)
                    } else {
                        Ok(EvalValue::Constant(c / x))
                    }
                }
                EvalValue::PreDice(d) => {
                    let d: Distribution = d.into();

                    if d.0.iter().any(|(x, _)| *x == 0) {
                        Err(EvalError::DivideByZero)
                    } else {
                        Ok(EvalValue::PostDice(d.mutate(|x| (c as Int) / x)))
                    }
                }
                EvalValue::PostDice(d) => {
                    if d.0.iter().any(|(x, _)| *x == 0) {
                        Err(EvalError::DivideByZero)
                    } else {
                        Ok(EvalValue::PostDice(d.mutate(|x| (c as Int) / x)))
                    }
                }
            },
            (Value::Operator(o), Value::Default) => o.as_ref().clone().eval(),
            (Value::Operator(o), Value::Constant(c)) => {
                if c == 0 {
                    Err(EvalError::DivideByZero)
                } else {
                    Ok(match o.as_ref().clone().eval()? {
                        EvalValue::Constant(x) => EvalValue::Constant(x / c),
                        EvalValue::PreDice(d) => {
                            EvalValue::PostDice(Distribution::from(d).mutate(|x| x / (c as Int)))
                        }
                        EvalValue::PostDice(d) => EvalValue::PostDice(d.mutate(|x| x / (c as Int))),
                    })
                }
            }
            (Value::Operator(l), Value::Operator(r)) => {
                match (l.as_ref().clone().eval()?, r.as_ref().clone().eval()?) {
                    (EvalValue::Constant(l), EvalValue::Constant(r)) => {
                        if r == 0 {
                            Err(EvalError::DivideByZero)
                        } else {
                            Ok(EvalValue::Constant(l / r))
                        }
                    }
                    (EvalValue::Constant(c), EvalValue::PreDice(d)) => {
                        let d: Distribution = d.into();
                        if d.0.iter().any(|(x, _)| *x == 0) {
                            Err(EvalError::DivideByZero)
                        } else {
                            Ok(EvalValue::PostDice(d.mutate(|x| (c as Int) / x)))
                        }
                    }
                    (EvalValue::Constant(c), EvalValue::PostDice(d)) => {
                        if d.0.iter().any(|(x, _)| *x == 0) {
                            Err(EvalError::DivideByZero)
                        } else {
                            Ok(EvalValue::PostDice(d.mutate(|x| (c as Int) / x)))
                        }
                    }
                    (EvalValue::PreDice(d), EvalValue::Constant(c)) => {
                        if c == 0 {
                            Err(EvalError::DivideByZero)
                        } else {
                            Ok(EvalValue::PostDice(
                                Distribution::from(d).mutate(|x| x / (c as Int)),
                            ))
                        }
                    }
                    (EvalValue::PostDice(d), EvalValue::Constant(c)) => {
                        if c == 0 {
                            Err(EvalError::DivideByZero)
                        } else {
                            Ok(EvalValue::PostDice(d.mutate(|x| x / (c as Int))))
                        }
                    }
                    _ => Err(EvalError::DivideDiceWithDice),
                }
            }
        }
    }
}

impl Mul for Value {
    type Output = Result<EvalValue, EvalError>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Default, Value::Default) | (Value::Operator(_), Value::Default) => {
                Ok(EvalValue::Constant(1))
            }
            (Value::Default, Value::Constant(c)) | (Value::Constant(c), Value::Default) => {
                Ok(EvalValue::Constant(c))
            }
            (Value::Default, Value::Operator(o)) => o.as_ref().clone().eval(),
            (Value::Constant(l), Value::Constant(r)) => Ok(EvalValue::Constant(l * r)),
            (Value::Constant(c), Value::Operator(o)) | (Value::Operator(o), Value::Constant(c)) => {
                match o.as_ref().clone().eval()? {
                    EvalValue::Constant(x) => Ok(EvalValue::Constant(c * x)),
                    EvalValue::PreDice(d) => Ok(EvalValue::PostDice(
                        Distribution::from(d).mutate(|x| x * (c as Int)),
                    )),
                    EvalValue::PostDice(d) => Ok(EvalValue::PostDice(d.mutate(|x| x * (c as Int)))),
                }
            }
            (Value::Operator(l), Value::Operator(r)) => {
                match (l.as_ref().clone().eval()?, r.as_ref().clone().eval()?) {
                    (EvalValue::Constant(l), EvalValue::Constant(r)) => {
                        Ok(EvalValue::Constant(l * r))
                    }
                    (EvalValue::Constant(c), EvalValue::PreDice(d))
                    | (EvalValue::PreDice(d), EvalValue::Constant(c)) => Ok(EvalValue::PostDice(
                        Distribution::from(d).mutate(|x| x * (c as Int)),
                    )),
                    (EvalValue::Constant(c), EvalValue::PostDice(d))
                    | (EvalValue::PostDice(d), EvalValue::Constant(c)) => {
                        Ok(EvalValue::PostDice(d.mutate(|x| x * (c as Int))))
                    }
                    _ => Err(EvalError::MultiplyDiceWithDice),
                }
            }
        }
    }
}

impl Sub for Value {
    type Output = Result<EvalValue, EvalError>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Default, Value::Default) => Ok(EvalValue::Constant(0)),
            (Value::Default, Value::Constant(c)) => Ok(EvalValue::Constant(-c)),
            (Value::Default, Value::Operator(c)) => match c.as_ref().clone().eval()? {
                EvalValue::Constant(c) => Ok(EvalValue::Constant(-c)),
                EvalValue::PreDice(d) => {
                    Ok(EvalValue::PostDice(Distribution::from(d).mutate(|x| -x)))
                }
                EvalValue::PostDice(d) => Ok(EvalValue::PostDice(d.mutate(|x| -x))),
            },
            (Value::Constant(c), Value::Default) => Ok(EvalValue::Constant(c)),
            (Value::Constant(l), Value::Constant(r)) => Ok(EvalValue::Constant(l - r)),
            (Value::Constant(c), Value::Operator(o)) => match o.as_ref().clone().eval()? {
                EvalValue::Constant(x) => Ok(EvalValue::Constant(c - x)),
                EvalValue::PreDice(d) => Ok(EvalValue::PostDice(
                    Distribution::from(d).mutate(|x| (c as Int) - x),
                )),
                EvalValue::PostDice(d) => Ok(EvalValue::PostDice(d.mutate(|x| (c as Int) - x))),
            },
            (Value::Operator(o), Value::Default) => o.as_ref().clone().eval(),
            (Value::Operator(o), Value::Constant(c)) => match o.as_ref().clone().eval()? {
                EvalValue::Constant(x) => Ok(EvalValue::Constant(x - c)),
                EvalValue::PreDice(d) => Ok(EvalValue::PostDice(
                    Distribution::from(d).mutate(|x| x - (c as Int)),
                )),
                EvalValue::PostDice(d) => Ok(EvalValue::PostDice(d.mutate(|x| x - (c as Int)))),
            },
            (Value::Operator(l), Value::Operator(r)) => {
                match (l.as_ref().clone().eval()?, r.as_ref().clone().eval()?) {
                    (EvalValue::Constant(l), EvalValue::Constant(r)) => {
                        Ok(EvalValue::Constant(l - r))
                    }
                    (EvalValue::Constant(c), EvalValue::PreDice(d)) => Ok(EvalValue::PostDice(
                        Distribution::from(d).mutate(|x| (c as Int) - x),
                    )),
                    (EvalValue::Constant(c), EvalValue::PostDice(d)) => {
                        Ok(EvalValue::PostDice(d.mutate(|x| (c as Int) - x)))
                    }
                    (EvalValue::PreDice(d), EvalValue::Constant(c)) => Ok(EvalValue::PostDice(
                        Distribution::from(d).mutate(|x| x - (c as Int)),
                    )),
                    (EvalValue::PreDice(l), EvalValue::PreDice(r)) => Ok(EvalValue::PostDice(
                        Distribution::from(l) - Distribution::from(r),
                    )),
                    (EvalValue::PreDice(l), EvalValue::PostDice(r)) => {
                        Ok(EvalValue::PostDice(Distribution::from(l) - r))
                    }
                    (EvalValue::PostDice(d), EvalValue::Constant(c)) => {
                        Ok(EvalValue::PostDice(d.mutate(|x| x - (c as Int))))
                    }
                    (EvalValue::PostDice(l), EvalValue::PreDice(r)) => {
                        Ok(EvalValue::PostDice(l - Distribution::from(r)))
                    }
                    (EvalValue::PostDice(l), EvalValue::PostDice(r)) => {
                        Ok(EvalValue::PostDice(l - r))
                    }
                }
            }
        }
    }
}

impl Add for Value {
    type Output = Result<EvalValue, EvalError>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Default, Value::Default) => Ok(EvalValue::Constant(0)),
            (Value::Default, Value::Constant(x)) | (Value::Constant(x), Value::Default) => {
                Ok(EvalValue::Constant(x))
            }
            (Value::Default, Value::Operator(x)) | (Value::Operator(x), Value::Default) => {
                x.as_ref().clone().eval()
            }
            (Value::Constant(l), Value::Constant(r)) => Ok(EvalValue::Constant(l + r)),
            (Value::Constant(c), Value::Operator(o)) | (Value::Operator(o), Value::Constant(c)) => {
                match o.as_ref().clone().eval()? {
                    EvalValue::Constant(x) => Ok(EvalValue::Constant(x + c)),
                    EvalValue::PreDice(d) => Ok(EvalValue::PostDice(
                        Distribution::from(d).mutate(|x| x + (c as Int)),
                    )),
                    EvalValue::PostDice(d) => Ok(EvalValue::PostDice(d.mutate(|x| x + (c as Int)))),
                }
            }
            (Value::Operator(l), Value::Operator(r)) => {
                match (l.as_ref().clone().eval()?, r.as_ref().clone().eval()?) {
                    (EvalValue::Constant(l), EvalValue::Constant(r)) => {
                        Ok(EvalValue::Constant(l + r))
                    }
                    (EvalValue::Constant(c), EvalValue::PreDice(d))
                    | (EvalValue::PreDice(d), EvalValue::Constant(c)) => Ok(EvalValue::PostDice(
                        Distribution::from(d).mutate(|x| (c as Int) + x),
                    )),
                    (EvalValue::Constant(c), EvalValue::PostDice(d))
                    | (EvalValue::PostDice(d), EvalValue::Constant(c)) => {
                        Ok(EvalValue::PostDice(d.mutate(|x| (c as Int) + x)))
                    }
                    (EvalValue::PreDice(pre), EvalValue::PostDice(post))
                    | (EvalValue::PostDice(post), EvalValue::PreDice(pre)) => {
                        Ok(EvalValue::PostDice(Distribution::from(pre) + post))
                    }
                    (EvalValue::PreDice(l), EvalValue::PreDice(r)) => Ok(EvalValue::PostDice(
                        Distribution::from(l) + Distribution::from(r),
                    )),
                    (EvalValue::PostDice(l), EvalValue::PostDice(r)) => {
                        Ok(EvalValue::PostDice(l + r))
                    }
                }
            }
        }
    }
}
