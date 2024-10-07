use super::value::Value;

#[derive(Debug)]
pub struct Local {
    value: Value,
}

impl Local {
    pub fn from_i32(value: i32) -> Local {
        Local {
            value: Value::I32(value),
        }
    }
    pub fn from_i32_default() -> Local {
        Local {
            value: Value::I32(0),
        }
    }
}
