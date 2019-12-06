use crate::postgres::error::{Error};
use crate::postgres::query_field::{QueryWithParamsLoc};
use crate::postgres::query_token::{QueryTokens};
use crate::postgres::static_constant::FIELD_ESCAPE;

#[derive(Debug)]
pub struct QuerySets(Vec<(String, QueryTokens)>);
impl QuerySets {
    pub fn build_for_create(&self, i: &mut i8) -> Result<QueryWithParamsLoc, Error> {
        let mut v1: Vec<String> = Vec::new();
        let mut v2: Vec<String> = Vec::new();
        let mut p: Vec<usize> = Vec::new();
        for (field, qtoken) in &self.0 {
            v1.push(format!("{}{}{}", FIELD_ESCAPE, field, FIELD_ESCAPE));
            let built = qtoken.build(i)?;
            v2.push(built.query);
            p.extend(built.parameters_loc);
        }
        Ok(QueryWithParamsLoc {
            query: format!("({}) VALUES ({})", v1.join(","), v2.join(",")),
            parameters_loc: p
        })
    }
    pub fn build_for_update(&self, i: &mut i8) -> Result<QueryWithParamsLoc, Error> {
        let mut v: Vec<String> = Vec::new();
        let mut p: Vec<usize> = Vec::new();
        for (field, qtoken) in &self.0 {
            let built = qtoken.build(i)?;
            v.push(format!("{}{}{}={}", FIELD_ESCAPE, field, FIELD_ESCAPE, built.query));
            p.extend(built.parameters_loc);
        }
        Ok(QueryWithParamsLoc {
            query: v.join(","),
            parameters_loc: p
        })
    }
    pub fn set(&mut self, field: (String, QueryTokens)) -> &mut Self {
        self.0.push(field);
        self
    }
}

impl Default for QuerySets {
    fn default() -> Self {
        QuerySets(Vec::new())
    }
}
