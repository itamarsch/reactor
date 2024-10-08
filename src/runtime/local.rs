use super::value::Value;

#[derive(Debug)]
pub struct Local {
    value: Value,
}

impl Local {
    pub fn value(&self) -> Value {
        self.value
    }

    pub fn set_value(&mut self, value: Value) {
        match (&mut self.value, value) {
            (Value::I32(ref mut val), Value::I32(new_value)) => {
                *val = new_value;
            }
            (Value::I64(ref mut val), Value::I64(new_value)) => {
                *val = new_value;
            }
            (Value::F32(ref mut val), Value::F32(new_value)) => {
                *val = new_value;
            }
            (Value::F64(ref mut val), Value::F64(new_value)) => {
                *val = new_value;
            }

            _ => {
                panic!(
                    "Tried setting local with invalid type: Current: {:?}, New: {:?}",
                    self.value, value
                );
            }
        }
    }

    pub fn from_i32(value: i32) -> Local {
        Local {
            value: Value::I32(value),
        }
    }

    pub fn from_i64(value: i64) -> Local {
        Local {
            value: Value::I64(value),
        }
    }

    pub fn from_f32(value: f32) -> Local {
        Local {
            value: Value::F32(value),
        }
    }

    pub fn from_f64(value: f64) -> Local {
        Local {
            value: Value::F64(value),
        }
    }

    pub fn from_i32_default() -> Local {
        Local {
            value: Value::I32(0),
        }
    }
    pub fn from_i64_default() -> Local {
        Local {
            value: Value::I64(0),
        }
    }
    pub fn from_f32_default() -> Local {
        Local {
            value: Value::F32(0.0),
        }
    }
    pub fn from_f64_default() -> Local {
        Local {
            value: Value::F64(0.0),
        }
    }
}
