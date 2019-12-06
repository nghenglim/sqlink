// #[derive(Clone, Debug, PartialEq)]
// pub enum ParameterValue {
//     Blob(Vec<u8>),
//     String(String),
//     Bool(bool),
//     I64(i64),
//     I32(i32),
//     I16(i16),
//     I8(i8),
//     F64(f64),
//     Null,
// }

// macro_rules! from_option(
//     ($t:ty) => (
//         impl From<Option<$t>> for ParameterValue {
//             fn from(item: Option<$t>) -> Self {
//                 match item {
//                     None => ParameterValue::Null,
//                     Some(val) => val.into(),
//                 }
//             }
//         }
//     )
// );

// macro_rules! to_val(
//     ($id:ident, $t:ty) => (
//         impl From<$t> for ParameterValue {
//             fn from(item: $t) -> Self {
//                 ParameterValue::$id(item)
//             }
//         }
//     )
// );
// to_val!(I64, i64);
// to_val!(I32, i32);
// to_val!(I16, i16);
// to_val!(I8, i8);
// to_val!(F64, f64);
// to_val!(String, String);
// to_val!(Bool, bool);
// impl From<&str> for ParameterValue {
//     fn from(item: &str) -> Self {
//         ParameterValue::String(item.to_owned())
//     }
// }
// impl From<Vec<u8>> for ParameterValue {
//     fn from(item: Vec<u8>) -> Self {
//         ParameterValue::Blob(item)
//     }
// }
// from_option!(&str);
// from_option!(String);
// from_option!(bool);
// from_option!(Vec<u8>);

pub type ParameterValue<'a> = Box<dyn postgres::types::ToSql + 'a>;
pub type ParameterValueAsRef<'a> = &'a dyn postgres::types::ToSql;
pub struct QueryWithParamsLoc {
    pub query: String,
    pub parameters_loc: Vec<usize>,
}
pub struct QueryWithParams<'a> {
    pub query: String,
    pub parameters: Vec<ParameterValueAsRef<'a>>,
}
