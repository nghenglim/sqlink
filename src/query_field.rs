#[derive(Clone, Debug)]
pub enum FieldValue {
    RawSql(String),
    String(String),
    Double(f64),
    Long(i64),
    Boolean(bool),
}

#[derive(Clone, Debug)]
pub struct SetField {
    name: String,
    val: FieldValue,
}
