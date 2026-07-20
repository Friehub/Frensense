pub enum Value {
    Integer(i32),
    Float(f64),
}

pub fn read_value(v: Value) -> i32 {
    match v {
        Value::Integer(i) => i,
        Value::Float(f) => f as i32,
    }
}
