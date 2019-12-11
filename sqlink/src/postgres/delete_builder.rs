use crate::error::Error;
use crate::postgres::query_table::{QueryTables, QueryTable};
use crate::postgres::query_field::{QueryWithParams, ParameterValueAsRef};
use crate::postgres::query_token::{FormatQueryTup};
use crate::postgres::query_where::{QueryWheres, WhereOperator};

#[derive(Default, Debug)]
pub struct SqlDelete<'a> {
    _tables: QueryTables,
    _wheres: QueryWheres,
    _parameters: Vec<ParameterValueAsRef<'a>>,
}

impl<'a> SqlDelete<'a> {
    pub fn new() -> SqlDelete<'static> {
        SqlDelete::default()
    }
    pub fn build(&self) -> Result<QueryWithParams, Error> {
        if self._tables.len() != 1 {
            return Err(Error::Syntax("currently only support 1 table for DELETE".to_owned()))
        }
        let mut param_iter = 1;
        let built_for_table = self._tables.build(&mut param_iter)?;
        let mut vec: Vec<String> = Vec::new();
        let mut p: Vec<ParameterValueAsRef> = Vec::new();
        for ploc in built_for_table.parameters_loc {
            p.push(self._parameters[ploc]);
        }
        vec.push(format!("DELETE FROM {}", built_for_table.query));
        if self._wheres.len() > 0 {
            let built_for_where = self._wheres.build(&mut param_iter)?;
            vec.push(format!("WHERE {}", built_for_where.query));
            for ploc in built_for_where.parameters_loc {
                p.push(self._parameters[ploc]);
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
    use crate::postgres::query_token::{format_query};
    use super::*;
    #[test]
    fn test_update_builder_1() {
        let mut sql_delete = SqlDelete::new();
        let qbuild = sql_delete
            .table("user")
            .and_where(format_query("id = {}".to_owned(), vec![&(1)]))
            .build().unwrap();
        assert_eq!(qbuild.query, "DELETE FROM \"user\" WHERE id = $1");
    }
}
