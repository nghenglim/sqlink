use crate::error::Error;
use crate::postgres::query_table::{QueryTables, QueryTable};
use crate::postgres::query_field::{QueryWithParams, ParameterValue, ParameterValueAsRef};
use crate::postgres::query_token::{QueryTokens, QueryToken, FormatQueryTup};
use crate::postgres::query_set::{QuerySets};
use crate::postgres::query_return::{QueryReturns, QueryReturnField};

#[derive(Default, Debug)]
pub struct SqlInsert<'a> {
    _tables: QueryTables, // to support update tableA, tableB set ...
    _intos: QuerySets,
    _returns: QueryReturns,
    _parameters: Vec<ParameterValue<'a>>,
}

impl<'a> SqlInsert<'a> {
    pub fn new() -> SqlInsert<'static> {
        SqlInsert::default()
    }
    pub fn build(&self) -> Result<QueryWithParams, Error> {
        let mut param_iter = 1;
        if self._tables.len() != 1 {
            return Err(Error::Syntax("insert can only have 1 table".to_owned()))
        }
        let mut vec: Vec<String> = Vec::new();
        let built_for_table = self._tables.build(&mut param_iter)?;
        let built_for_create = self._intos.build_for_create(&mut param_iter)?;
        let mut p: Vec<ParameterValueAsRef> = Vec::new();
        for ploc in built_for_table.parameters_loc {
            p.push(self._parameters[ploc].as_ref());
        }
        for ploc in built_for_create.parameters_loc {
            p.push(self._parameters[ploc].as_ref());
        }
        vec.push(format!("INSERT INTO {}{}", built_for_table.query, built_for_create.query));
        if self._returns.len() > 0 {
            let built_for_return: String = self._returns.build()?;
            vec.push(format!("RETURNING {}", built_for_return));
        }
        Ok(QueryWithParams {
            query: vec.join(" "),
            parameters: p,
        })
    }
    pub fn returning<S: Into<QueryReturnField>>(&mut self, field: S) -> &mut Self {
        self._returns.push(field.into());
        self
    }
    pub fn table<S: Into<QueryTable>>(&mut self, table: S) -> &mut Self {
        self._tables.push(table.into());
        self
    }
    pub fn into<S: Into<String>, T>(&mut self, field: S, param: T) -> &mut Self where T: postgres::types::ToSql + 'a {
        self._parameters.push(Box::new(param));
        self._intos.set((field.into(), QueryTokens(vec![QueryToken::ParameterLoc(self._parameters.len() - 1)])));
        self
    }
    pub fn into_raw<S: Into<String>>(&mut self, field: S, tup: FormatQueryTup<'a>) -> &mut Self{
        let len = self._parameters.len();
        self._parameters.extend(tup.1);
        let qtokens = (tup.0).to_query_tokens(len);
        self._intos.set((field.into(), qtokens));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::postgres::query_token::{format_query};
    use super::*;
    #[test]
    fn test_insert_builder_1() {
        let user_spouse: Option<String> = None;
        let mut sql_insert = SqlInsert::new();
        let qbuild = sql_insert
            .returning("id")
            .returning("age")
            .table("user")
            .into("spouse", user_spouse)
            .into("age", 1337)
            .into_raw("name", format_query("LOWER({})".to_owned(), vec![Box::new("foo".to_owned())]))
            .build().unwrap();
        assert_eq!(qbuild.query, "INSERT INTO \"user\"(\"spouse\",\"age\",\"name\") VALUES ($1,$2,LOWER($3)) RETURNING id, age");
        // println!("{:?}", qbuild.parameters);
        // assert_eq!(qbuild.parameters, vec![ParameterValue::Null, ParameterValue::I32(1337), ParameterValue::String("foo".to_owned())]);
    }
}
