use super::value::Value;

pub struct ValueStack {
    values: Vec<Value>,
}

impl ValueStack {
    pub fn empty() -> ValueStack {
        Self { values: vec![] }
    }
}
