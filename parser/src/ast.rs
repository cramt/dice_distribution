use std::{borrow::Cow, error::Error, fmt::Display, ops::Deref, rc::Rc, str::FromStr};

use once_cell::sync::Lazy;
use regex::Regex;
use shoulda::Shoulda;

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use shoulda::Shoulda;

    use super::{Operator, Value};

    #[test]
    fn parse_plus() {
        let value: Value = "2 + 2".parse().unwrap();
        value.should().eq(Value::Operator(Rc::new(Operator::Plus(
            Value::Constant(2),
            Value::Constant(2),
        ))));
    }

    #[test]
    fn parse_dice() {
        let value: Value = "2d6".parse().unwrap();
        value.should().eq(Value::Operator(Rc::new(Operator::Dice(
            Value::Constant(2),
            Value::Constant(6),
        ))));
    }

    #[test]
    fn parse_kh() {
        let value: Value = "2d6kh2".parse().unwrap();
        value
            .should()
            .eq(Value::Operator(Rc::new(Operator::KeepHighest(
                Value::Operator(Rc::new(Operator::Dice(
                    Value::Constant(2),
                    Value::Constant(6),
                ))),
                Value::Constant(2),
            ))));
    }

    #[test]
    fn parse_default_dice() {
        let value: Value = "d6".parse().unwrap();
        value.should().eq(Value::Operator(Rc::new(Operator::Dice(
            Value::Default,
            Value::Constant(6),
        ))));
    }

    #[cfg(test)]
    mod weird_syntax {
        use std::rc::Rc;

        use shoulda::Shoulda;

        use crate::ast::{Operator, Value};

        #[test]
        fn plus_with_negative_number() {
            let value: Value = "2+-3".parse().unwrap();
            value.should().eq(Value::Operator(Rc::new(Operator::Plus(
                Value::Constant(2),
                Value::Constant(-3),
            ))));
        }
    }

    #[cfg(test)]
    mod error {
        use shoulda::Shoulda;

        use crate::ast::{Value, ValueParseError};

        #[test]
        fn invalid_parentheses() {
            let value = "(((()))".parse::<Value>();
            value.should().eq(Err(ValueParseError::InvalidParentheses));
        }

        #[test]
        fn invalid_ops() {
            let value = "2y6".parse::<Value>();
            value.should().eq(Err(ValueParseError::InvalidOperators(
                vec!["y".to_string()],
            )));
        }
    }

    #[cfg(test)]
    mod order_of_op {
        use std::rc::Rc;

        use shoulda::Shoulda;

        use super::{Operator, Value};

        #[test]
        fn parentheses() {
            let value: Value = "(2 + 2) * (5 + 3)".parse().unwrap();
            value
                .should()
                .eq(Value::Operator(Rc::new(Operator::Multiply(
                    Value::Operator(Rc::new(Operator::Plus(
                        Value::Constant(2),
                        Value::Constant(2),
                    ))),
                    Value::Operator(Rc::new(Operator::Plus(
                        Value::Constant(5),
                        Value::Constant(3),
                    ))),
                ))));
        }
    }
}

#[derive(Debug, Shoulda)]
pub enum ValueParseError {
    InvalidOperators(Vec<String>),
    InvalidParentheses,
}

impl Display for ValueParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueParseError::InvalidOperators(c) => {
                let error = c
                    .iter()
                    .map(|x| format!("Invalid Operator: {x}\n"))
                    .collect::<String>();
                write!(f, "{error}")
            }
            ValueParseError::InvalidParentheses => write!(f, "Invalid Parentheses"),
        }
    }
}

impl Error for ValueParseError {}

#[derive(Debug, Shoulda, Clone)]
pub enum Value {
    Default,
    Constant(i32),
    Operator(Rc<Operator>),
}

fn order_of_operations() -> &'static Vec<(&'static str, Regex)> {
    const ORDER: [&str; 8] = ["+", "-", "*", "/", "cs<=", "cs<", "kh", "d"];
    const ANYTHING_WITH_VALID_PARENTHESES: &str = r"([^\(\)]*(?:\((?:.|\s)*\))*[^\(\)]*)";
    static LAZY: Lazy<Vec<(&'static str, Regex)>> = Lazy::new(|| {
        ORDER
            .iter()
            .map(|x| (x, regex::escape(x)))
            .map(|(x, y)| {
                (
                    x,
                    format!(
                        "^{ANYTHING_WITH_VALID_PARENTHESES}{y}{ANYTHING_WITH_VALID_PARENTHESES}$"
                    ),
                )
            })
            .map(|(x, y)| (*x, Regex::new(y.as_str()).unwrap()))
            .collect()
    });
    LAZY.deref()
}

impl FromStr for Value {
    type Err = ValueParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Ok(Self::Default);
        }
        static TRIM_PARENTHESIS_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^\([^\(\)]*(?:\((?:.|\s)*\))*[^\(\)]*\)$").unwrap());
        if TRIM_PARENTHESIS_REGEX.is_match(s) {
            let s = &s[1..(s.len() - 1)];
            return Self::from_str(s);
        }
        if let Ok(con) = s.parse::<i32>() {
            return Ok(Self::Constant(con));
        }

        for (op, regex) in order_of_operations() {
            if let Some(caps) = regex.captures(s) {
                let l = caps.get(1).unwrap().as_str();
                let r = caps.get(2).unwrap().as_str();
                let l = l.parse()?;
                let r = r.parse()?;
                return Ok(Self::Operator(Rc::new(Operator::new(op, l, r).unwrap())));
            }
        }
        static VALID_CHAR: Lazy<Regex> = Lazy::new(|| {
            Regex::new(
                format!(
                    "(?:{})",
                    order_of_operations()
                        .iter()
                        .map(|(x, _)| Cow::Borrowed(*x))
                        .chain(('0'..='9').map(|x| Cow::Owned(x.to_string())))
                        .map(|x| regex::escape(x.deref()))
                        .collect::<Vec<_>>()
                        .join("|")
                )
                .as_str(),
            )
            .unwrap()
        });
        if s.chars().filter(|x| *x == '(').count() != s.chars().filter(|x| *x == ')').count() {
            Err(ValueParseError::InvalidParentheses)
        } else {
            let invalid_ops = VALID_CHAR
                .replace_all(s, " ")
                .split_ascii_whitespace()
                .filter(|x| !x.is_empty())
                .map(|x| x.to_string())
                .collect::<Vec<_>>();
            Err(ValueParseError::InvalidOperators(invalid_ops))
        }
    }
}

#[derive(Debug, Shoulda, Clone)]
pub enum Operator {
    Plus(Value, Value),
    Minus(Value, Value),
    Multiply(Value, Value),
    Divide(Value, Value),
    KeepHighest(Value, Value),
    CountSuccesses(Value, Value),
    Dice(Value, Value),
}

impl Operator {
    pub fn new(op: &str, l: Value, r: Value) -> Option<Self> {
        match op {
            "cs<" => Some(Self::CountSuccesses(l, r)),
            "cs<=" => Some(Self::CountSuccesses(
                l,
                Value::Operator(Rc::new(Self::Plus(r, Value::Constant(1)))),
            )),
            "kh" => Some(Self::KeepHighest(l, r)),
            "d" => Some(Self::Dice(l, r)),
            "*" => Some(Self::Multiply(l, r)),
            "/" => Some(Self::Divide(l, r)),
            "-" => Some(Self::Minus(l, r)),
            "+" => Some(Self::Plus(l, r)),
            _ => None,
        }
    }
}
