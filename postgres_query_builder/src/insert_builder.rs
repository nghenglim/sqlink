use crate::error::Error;
use crate::query_table::{QueryTables, QueryTable};
use crate::query_field::{QueryWithParams, ParameterValue, ParameterValueAsRef};
use crate::query_token::{QueryTokens, QueryToken, FormatQueryTup};
use crate::query_set::{QuerySets};

#[derive(Default, Debug)]
pub struct SqlInsert<'a> {
    _tables: QueryTables, // to support update tableA, tableB set ...
    _sets: QuerySets,
    _parameters: Vec<ParameterValue<'a>>,
}

impl<'a> SqlInsert<'a> {
    pub fn new() -> SqlInsert<'a> {
        SqlInsert::default()
    }
    pub fn build(&self) -> Result<QueryWithParams, Error> {
        let mut param_iter = 1;
        if self._tables.len() != 1 {
            return Err(Error::Syntax("insert can only have 1 table".to_owned()))
        }
        let built_for_table = self._tables.build(&mut param_iter)?;
        let built_for_create = self._sets.build_for_create(&mut param_iter)?;
        let mut p: Vec<ParameterValueAsRef> = Vec::new();
        for ploc in built_for_table.parameters_loc {
            p.push(self._parameters[ploc].as_ref());
        }
        for ploc in built_for_create.parameters_loc {
            p.push(self._parameters[ploc].as_ref());
        }
        Ok(QueryWithParams {
            query: format!("INSERT INTO {}{}", built_for_table.query, built_for_create.query),
            parameters: p,
        })
    }
    pub fn table<S: Into<QueryTable>>(&mut self, table: S) -> &mut Self {
        self._tables.push(table.into());
        self
    }
    pub fn into<S: Into<String>, T>(&mut self, field: (S, T)) -> &mut Self where T: postgres::types::ToSql + 'a {
        self._parameters.push(Box::new(field.1));
        self._sets.set((field.0.into(), QueryTokens(vec![QueryToken::ParameterLoc(self._parameters.len() - 1)])));
        self
    }
    pub fn into_raw<S: Into<String>>(&mut self, tup: (S, FormatQueryTup<'a>)) -> &mut Self{
        let len = self._parameters.len();
        self._parameters.extend((tup.1).1);
        let qtokens = ((tup.1).0).to_query_tokens(len);
        self._sets.set((tup.0.into(), qtokens));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::query_token::{format_query};
    use super::*;
    #[test]
    fn test_insert_builder_1() {
        let user_spouse: Option<String> = None;
        let mut sql_insert = SqlInsert::new();
        let qbuild = sql_insert
            .table("user")
            .into(("spouse", user_spouse))
            .into(("age", 1337))
            .into_raw(("name", format_query("LOWER({})".to_owned(), vec![Box::new("foo".to_owned())])))
            .build().unwrap();
        assert_eq!(qbuild.query, "INSERT INTO \"user\"(\"spouse\",\"age\",\"name\") VALUES ($1,$2,LOWER($3))");
        // println!("{:?}", qbuild.parameters);
        // assert_eq!(qbuild.parameters, vec![ParameterValue::Null, ParameterValue::I32(1337), ParameterValue::String("foo".to_owned())]);
    }
}
