use std::{error::Error, fmt::Display, str::FromStr};

use js_sys::Map;
use model::distribution::Distribution;
use parser::{ast::ValueParseError, eval::EvalError};
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub enum DistError {
    Eval(EvalError),
    Parse(ValueParseError),
}

impl Display for DistError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DistError::Eval(eval) => write!(f, "Eval Error: {eval}"),
            DistError::Parse(parse) => write!(f, "Parse Error: {parse}"),
        }
    }
}

impl Error for DistError {}

#[wasm_bindgen]
pub fn eval(s: &str) -> Result<Map, String> {
    let ast = parser::ast::Value::from_str(s)
        .map_err(DistError::Parse)
        .map_err(|x| x.to_string())?;
    let dist: Distribution = ast
        .eval()
        .map_err(DistError::Eval)
        .map_err(|x| x.to_string())?
        .into();
    let map = Map::new();
    for (k, v) in dist.0 {
        map.set(&JsValue::from_f64(k as f64), &JsValue::from_f64(v as f64));
    }
    Ok(map)
}
