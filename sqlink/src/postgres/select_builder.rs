use crate::postgres::error::Error;
use crate::postgres::query_limit_offset::QueryLimitOffset;
use crate::postgres::query_where::{QueryWheres, WhereOperator};
use crate::postgres::query_order::{QueryOrders, QueryOrder};
use crate::postgres::query_table::{QueryTables, QueryTable};
use crate::postgres::query_select::{QuerySelects, QuerySelectField};
use crate::postgres::query_field::{QueryWithParams, ParameterValue, ParameterValueAsRef};
use crate::postgres::query_token::FormatQueryTup;

#[derive(Default, Debug)]
pub struct SqlSelect<'a> {
    _tables: QueryTables, // to support update tableA, tableB set ...
    _wheres: QueryWheres,
    _selects: QuerySelects,
    _orders: QueryOrders,
    _limit_offset: Option<QueryLimitOffset>,
    _parameters: Vec<ParameterValue<'a>>,
}

impl<'a> SqlSelect<'a> {
    pub fn new() -> SqlSelect<'static> {
        SqlSelect::default()
    }
    pub fn build(&self) -> Result<QueryWithParams, Error> {
        let mut param_iter = 1;
        let built_for_select: String = self._selects.build()?;
        let built_for_table = self._tables.build(&mut param_iter)?;
        let mut vec: Vec<String> = Vec::new();
        let mut p: Vec<ParameterValueAsRef> = Vec::new();
        vec.push(format!("SELECT {} FROM {}", built_for_select, built_for_table.query));
        for ploc in built_for_table.parameters_loc {
            p.push(self._parameters[ploc].as_ref());
        }
        if self._wheres.len() > 0 {
            let built_for_where = self._wheres.build(&mut param_iter)?;
            vec.push(format!("WHERE {}", built_for_where.query));
            for ploc in built_for_where.parameters_loc {
                p.push(self._parameters[ploc].as_ref());
            }
        }
        if self._orders.len() > 0 {
            let order: String = self._orders.build()?;
            vec.push(format!("ORDER BY {}", order));
        }
        if let Some(limitoffset) = &self._limit_offset {
            vec.push(limitoffset.build()?);
        }
        Ok(QueryWithParams {
            query: vec.join(" "),
            parameters: p,
        })
    }
    pub fn reset_fields(&mut self) -> &mut Self {
        self._selects = QuerySelects::default();
        self
    }
    pub fn select<S: Into<QuerySelectField>>(&mut self, field: S) -> &mut Self {
        self._selects.push(field.into());
        self
    }
    pub fn table<S: Into<QueryTable>>(&mut self, table: S) -> &mut Self {
        self._tables.push(table.into());
        self
    }
    pub fn order<S: Into<QueryOrder>>(&mut self, order: S) -> &mut Self {
        self._orders.push(order.into());
        self
    }
    pub fn limit_offset<S: Into<QueryLimitOffset>>(&mut self, limit_offset: S) -> &mut Self {
        self._limit_offset = Some(limit_offset.into());
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
    fn test_select_builder_1() {
        let mut sqlselect = SqlSelect::new();
        let qbuild = sqlselect
            .select("u.username")
            .select(("u.user_id", "uid"))
            .table(("user", "u"))
            .left_join(("user_detail", "ud"), format_query("u.user_id = ud.user_id AND ud.code = {}".to_owned(), vec![Box::new(2)]))
            .and_where(format_query("user.user_id = {}".to_owned(), vec![Box::new(1)]))
            .order(("user.created_at", "DESC"))
            .limit_offset((10, 20))
            .build().unwrap();
        assert_eq!(qbuild.query, "SELECT u.username, u.user_id AS uid FROM \"user\" AS u LEFT JOIN \"user_detail\" AS ud ON u.user_id = ud.user_id AND ud.code = $1 WHERE user.user_id = $2 ORDER BY user.created_at DESC LIMIT 10 OFFSET 20");
        // assert_eq!(qbuild.parameters, vec![ParameterValue::I32(2), ParameterValue::I32(1)]);
    }
}
