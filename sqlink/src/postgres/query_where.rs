use crate::error::Error;
use crate::postgres::query_field::{QueryWithParamsLoc};
use crate::postgres::query_token::{QueryTokens, QueryToken};
use crate::postgres::static_constant::PARAM_NOTATION;

#[derive(Debug)]
pub enum WhereOperator {
    Sql(String),
    ParameterLoc(usize),
    And,
    Or,
    Open,
    Close,
}

#[derive(Debug)]
pub struct QueryWheres(Vec<WhereOperator>);

impl Default for QueryWheres {
    fn default() -> Self {
        QueryWheres(Vec::new())
    }
}

impl From<QueryTokens> for QueryWheres {
    fn from(qtokens: QueryTokens) -> Self {
        let mut qw = Vec::with_capacity(qtokens.0.len());
        for token in qtokens.0 {
            let op = match token {
                QueryToken::Sql(s) => WhereOperator::Sql(s),
                QueryToken::ParameterLoc(p) => WhereOperator::ParameterLoc(p),
            };
            qw.push(op);
        }
        QueryWheres(qw)
    }
}

impl QueryWheres {
    pub fn build(&self, i: &mut i8) -> Result<QueryWithParamsLoc, Error> {
        let mut query = String::from("");
        let mut parameters = Vec::new();
        {
            for wo in &self.0 {
                match wo {
                    WhereOperator::Sql(s) => { query.push_str(&s); },
                    WhereOperator::ParameterLoc(p) => {
                        query.push_str(&format!("{}{}", PARAM_NOTATION, i));
                        parameters.push(p.clone());
                        *i += 1;
                    },
                    WhereOperator::And => { query.push_str(" AND "); },
                    WhereOperator::Or => { query.push_str(" OR "); },
                    WhereOperator::Open => { query.push_str("("); },
                    WhereOperator::Close => { query.push_str(")"); },
                }
            }
        }
        Ok(QueryWithParamsLoc {
            query: query,
            parameters_loc: parameters,
        })
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn push(&mut self, field: WhereOperator) {
        self.0.push(field);
    }
    pub fn extend(&mut self, qwhere: QueryWheres) {
        self.0.extend(qwhere.0);
    }
}
