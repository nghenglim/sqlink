use crate::postgres::query_field::{ParameterValueAsRef};
use crate::postgres::query_token::{format_query, TmpQueryTokens};
pub fn eq<S: Into<String>>(field: S, arg: ParameterValueAsRef) -> (TmpQueryTokens, Vec<ParameterValueAsRef>)  {
    format_query(format!("{} = {{}}", field.into()), vec![arg])
}
pub fn lt<S: Into<String>>(field: S, arg: ParameterValueAsRef) -> (TmpQueryTokens, Vec<ParameterValueAsRef>)  {
    format_query(format!("{} < {{}}", field.into()), vec![arg])
}
pub fn lte<S: Into<String>>(field: S, arg: ParameterValueAsRef) -> (TmpQueryTokens, Vec<ParameterValueAsRef>)  {
    format_query(format!("{} <= {{}}", field.into()), vec![arg])
}
pub fn gt<S: Into<String>>(field: S, arg: ParameterValueAsRef) -> (TmpQueryTokens, Vec<ParameterValueAsRef>)  {
    format_query(format!("{} > {{}}", field.into()), vec![arg])
}
pub fn gte<S: Into<String>>(field: S, arg: ParameterValueAsRef) -> (TmpQueryTokens, Vec<ParameterValueAsRef>)  {
    format_query(format!("{} >= {{}}", field.into()), vec![arg])
}
