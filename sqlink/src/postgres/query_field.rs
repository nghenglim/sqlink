pub type ParameterValueAsRef<'a> = &'a (dyn postgres_types::ToSql + std::marker::Sync);
pub struct QueryWithParamsLoc {
    pub query: String,
    pub parameters_loc: Vec<usize>,
}
pub struct QueryWithParams<'a> {
    pub query: String,
    pub parameters: Vec<ParameterValueAsRef<'a>>,
}
