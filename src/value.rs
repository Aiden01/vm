#[derive(Debug, Clone)]
pub enum Value<'a> {
    Float(f64),
    Int(i64),
    String(&'a str),
    Bool(bool),
    List(Vec<Value<'a>>)
}
