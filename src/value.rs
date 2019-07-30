use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum Value<'a> {
    Float(f64),
    Int(i64),
    String(&'a str),
    Bool(bool),
    List(Vec<Value<'a>>),
}

impl<'a> Display for Value<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            Value::Float(x) => write!(f, "{}", x),
            Value::Int(x) => write!(f, "{}", x),
            Value::String(x) => write!(f, "{}", x),
            Value::Bool(x) => write!(f, "{}", x),
            Value::List(values) => write!(f, "{:?}", values),
        }
    }
}
