use crate::error::Error;
use crate::query_table::{QueryTables, QueryTable};
use crate::query_field::{QueryWithParams, ParameterValue, ParameterValueAsRef};
use crate::query_token::{QueryTokens, QueryToken, FormatQueryTup};
use crate::query_where::{QueryWheres, WhereOperator};
use crate::query_set::{QuerySets};

#[derive(Default, Debug)]
pub struct SqlUpdate<'a> {
    _tables: QueryTables, // to support update tableA, tableB set ...
    _sets: QuerySets,
    _wheres: QueryWheres,
    _parameters: Vec<ParameterValue<'a>>,
}

impl<'a> SqlUpdate<'a> {
    pub fn new() -> SqlUpdate<'static> {
        SqlUpdate::default()
    }
    pub fn build(&self) -> Result<QueryWithParams, Error> {
        let mut param_iter = 1;
        let built_for_table = self._tables.build(&mut param_iter)?;
        let built_for_update = self._sets.build_for_update(&mut param_iter)?;
        let mut vec: Vec<String> = Vec::new();
        let mut p: Vec<ParameterValueAsRef> = Vec::new();
        for ploc in built_for_table.parameters_loc {
            p.push(self._parameters[ploc].as_ref());
        }
        for ploc in built_for_update.parameters_loc {
            p.push(self._parameters[ploc].as_ref());
        }
        vec.push(format!("UPDATE {} SET {}", built_for_table.query, built_for_update.query));
        if self._wheres.len() > 0 {
            let built_for_where = self._wheres.build(&mut param_iter)?;
            vec.push(format!("WHERE {}", built_for_where.query));
            for ploc in built_for_where.parameters_loc {
                p.push(self._parameters[ploc].as_ref());
            }
        }

        Ok(QueryWithParams {
            query: vec.join(" "),
            parameters: p,
        })
    }
    pub fn table<S: Into<QueryTable>>(&mut self, table: S) -> &mut Self {
        self._tables.push(table.into());
        self
    }
    pub fn set<S: Into<String>, T>(&mut self, field: (S, T)) -> &mut Self where T: postgres::types::ToSql + 'a {
        self._parameters.push(Box::new(field.1));
        self._sets.set((field.0.into(), QueryTokens(vec![QueryToken::ParameterLoc(self._parameters.len() - 1)])));
        self
    }
    pub fn set_raw<S: Into<String>>(&mut self, tup: (S, FormatQueryTup<'a>)) -> &mut Self{
        let len = self._parameters.len();
        self._parameters.extend((tup.1).1);
        let qtokens = ((tup.1).0).to_query_tokens(len);
        self._sets.set((tup.0.into(), qtokens));
        self
    }
    pub fn and_where(&mut self, ftup: FormatQueryTup<'a>) -> &mut Self {
        if self._wheres.len() > 0 {
            self._wheres.push(WhereOperator::And);
        }
        let len = self._parameters.len();
        self._parameters.extend(ftup.1);
        let qtokens = (ftup.0).to_query_tokens(len);
        self._wheres.extend(qtokens.into());
        self
    }
    pub fn or_where(&mut self, ftup: FormatQueryTup<'a>) -> &mut Self {
        if self._wheres.len() > 0 {
            self._wheres.push(WhereOperator::Or);
        }
        let len = self._parameters.len();
        self._parameters.extend(ftup.1);
        let qtokens = (ftup.0).to_query_tokens(len);
        self._wheres.extend(qtokens.into());
        self
    }
    pub fn and_where_open(&mut self) -> &mut Self {
        if self._wheres.len() > 0 {
            self._wheres.push(WhereOperator::And);
        }
        self._wheres.push(WhereOperator::Open);
        self
    }
    pub fn or_where_open(&mut self) -> &mut Self {
        if self._wheres.len() > 0 {
            self._wheres.push(WhereOperator::Or);
        }
        self._wheres.push(WhereOperator::Open);
        self
    }
    pub fn where_close(&mut self) -> &mut Self {
        self._wheres.push(WhereOperator::Close);
        // lets dont panic, if error database will panic
        self
    }
    pub fn inner_join<S: Into<QueryTable>>(&mut self, query_table: S, ftup: FormatQueryTup<'a>) -> &mut Self {
        let len = self._parameters.len();
        self._parameters.extend(ftup.1);
        let qtokens = (ftup.0).to_query_tokens(len);
        self._tables.inner_join(query_table.into());
        self._tables.on(qtokens.into());
        self
    }
    pub fn left_join<S: Into<QueryTable>>(&mut self, query_table: S, ftup: FormatQueryTup<'a>) -> &mut Self {
        let len = self._parameters.len();
        self._parameters.extend(ftup.1);
        let qtokens = (ftup.0).to_query_tokens(len);
        self._tables.left_join(query_table.into());
        self._tables.on(qtokens.into());
        self
    }
    pub fn right_join<S: Into<QueryTable>>(&mut self, query_table: S, ftup: FormatQueryTup<'a>) -> &mut Self {
        let len = self._parameters.len();
        self._parameters.extend(ftup.1);
        let qtokens = (ftup.0).to_query_tokens(len);
        self._tables.right_join(query_table.into());
        self._tables.on(qtokens.into());
        self
    }
    pub fn full_join<S: Into<QueryTable>>(&mut self, query_table: S, ftup: FormatQueryTup<'a>) -> &mut Self {
        let len = self._parameters.len();
        self._parameters.extend(ftup.1);
        let qtokens = (ftup.0).to_query_tokens(len);
        self._tables.full_join(query_table.into());
        self._tables.on(qtokens.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::query_token::{format_query};
    use super::*;
    #[test]
    fn test_update_builder_1() {
        let mut sqlupdate = SqlUpdate::new();
        let qbuild = sqlupdate
            .table("user")
            .set(("age", 1337))
            .set_raw(("name", format_query("LOWER({})".to_owned(), vec![Box::new("foo".to_owned())])))
            .and_where(format_query("id = {}".to_owned(), vec![Box::new(1)]))
            .build().unwrap();
        assert_eq!(qbuild.query, "UPDATE \"user\" SET \"age\"=$1,\"name\"=LOWER($2) WHERE id = $3");
        // assert_eq!(qbuild.parameters, vec![ParameterValue::I32(1337), ParameterValue::String("foo".to_owned()), ParameterValue::I32(1)]);
    }
}
