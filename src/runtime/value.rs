use crate::types::FuncIdx;

pub type Ref = Option<FuncIdx>;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Ref(Ref),
}
