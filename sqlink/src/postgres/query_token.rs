use crate::postgres::query_field::{ParameterValue, QueryWithParamsLoc};
use crate::postgres::static_constant::PARAM_NOTATION;
use crate::error::{Error};

#[derive(Debug)]
pub struct QueryTokens(pub Vec<QueryToken>);
impl QueryTokens {
    pub fn build(&self, i: &mut i8) -> Result<QueryWithParamsLoc, Error> {
        let mut query = String::from("");
        let mut parameters_loc = Vec::new();
        {
            for wo in &self.0 {
                match wo {
                    QueryToken::Sql(s) => { query.push_str(&s); },
                    QueryToken::ParameterLoc(p) => {
                        query.push_str(&format!("{}{}", PARAM_NOTATION, i));
                        parameters_loc.push(p.clone());
                        *i += 1;
                    },
                }
            }
        }
        Ok(QueryWithParamsLoc {
            query: query,
            parameters_loc: parameters_loc,
        })
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn push(&mut self, field: QueryToken) {
        self.0.push(field);
    }
    pub fn extend(&mut self, qwhere: QueryTokens) {
        self.0.extend(qwhere.0);
    }
}
impl Default for QueryTokens {
    fn default() -> Self {
        QueryTokens(Vec::new())
    }
}
#[derive(Debug)]
pub enum QueryToken {
    Sql(String),
    ParameterLoc(usize),
}
pub struct TmpQueryTokens(pub Vec<TmpQueryToken>);
#[derive(Debug)]
pub enum TmpQueryToken {
    Sql(String),
    Parameter,
}
impl TmpQueryTokens {
    // from will be the current paramvec.len()
    pub fn to_query_tokens(&self, from: usize) -> QueryTokens {
        let mut qtokens = QueryTokens::default();
        let mut iter = from;
        for i in &self.0 {
            match i {
                TmpQueryToken::Sql(s) => qtokens.push(QueryToken::Sql(s.clone())),
                TmpQueryToken::Parameter => {
                    qtokens.push(QueryToken::ParameterLoc(iter));
                    iter += 1;
                },
            }
        }
        qtokens
    }
}
pub type FormatQueryTup<'a> = (TmpQueryTokens, Vec<ParameterValue<'a>>);
pub fn format_query(query: String, arg: Vec<ParameterValue>) -> (TmpQueryTokens, Vec<ParameterValue>) {
    let mut argiter = 0;
    let mut qtoken: Vec<TmpQueryToken> = Vec::new();
    let queryvec: Vec<char> = query.chars().collect();
    let mut cur: usize = 0;
    let mut prevcur: usize = 0;
    while cur < queryvec.len() {
        if queryvec[cur] == '{' && cur + 1 < queryvec.len() {
            if queryvec[cur + 1] == '}' {
                if prevcur != cur {
                    let querypart: String = queryvec[prevcur..cur].iter().cloned().collect();
                    qtoken.push(TmpQueryToken::Sql(querypart));
                }
                qtoken.push(TmpQueryToken::Parameter); // in future will be checked with macro already
                argiter += 1;
                cur += 2;
                prevcur = cur;
            }
        } else {
            cur += 1;
        }
    }
    if prevcur != cur {
        let querypart: String = queryvec[prevcur..cur].iter().cloned().collect();
        qtoken.push(TmpQueryToken::Sql(querypart));
    }
    if arg.len() != argiter {
        panic!("argument len does not match with query");
    }
    (TmpQueryTokens(qtoken), arg)
}
