use crate::error::Error;
use crate::query_limit_offset::QueryLimitOffset;
use crate::query_where::{QueryWhere, WhereOperator};
use crate::query_order::QueryOrder;
use crate::query_table::QueryTable;
use crate::query_select_field::QuerySelectField;

#[derive(Default, Clone, Debug)]
pub struct SqlSelect {
    _tables: Vec<QueryTable>, // to support update tableA, tableB set ...
    _wheres: Vec<WhereOperator>,
    _selects: Vec<QuerySelectField>,
    _orders: Vec<QueryOrder>,
    _limit_offset: Option<QueryLimitOffset>,
}

#[derive(Clone, Debug)]
pub struct Qbuilt {
    query: String,
}

impl SqlSelect {
    pub fn new() -> SqlSelect {
        SqlSelect::default()
    }
    pub fn build(&self) -> Result<Qbuilt, Error> {
        let select: String = self._selects.clone().into_iter().map(|s| s.build()).collect::<Vec<String>>().join(", ");
        let table: String = self._tables.clone().into_iter().map(|s| s.build()).collect::<Vec<String>>().join(", ");
        let order: String = self._orders.clone().into_iter().map(|s| s.build()).collect::<Vec<String>>().join(", ");
        let qwhere: String = self._wheres.clone().into_iter().map(|s| s.build()).collect::<Vec<String>>().join(" ");
        let limit_offset: String = if let Some(limitoffset) = &self._limit_offset {
            format!(" {}", limitoffset.build())
        } else {
            " ".to_owned()
        };
        Ok(Qbuilt {
            query: format!("SELECT {} FROM {} WHERE {} ORDER BY {}{}", select, table, qwhere, order, limit_offset),
        })
    }
    pub fn reset_fields(&mut self) -> &mut Self {
        self._selects = Vec::new();
        self
    }
    pub fn select<S: Into<QuerySelectField>>(&mut self, field: S) -> &mut Self {
        self._selects.push(field.into());
        self
    }
    pub fn table<S: Into<QueryTable>>(&mut self, table: S) -> &mut Self {
        self._tables = vec![table.into()];
        self
    }
    pub fn order<S: Into<QueryOrder>>(&mut self, order: S) -> &mut Self {
        self._orders = vec![order.into()];
        self
    }
    pub fn limit_offset<S: Into<QueryLimitOffset>>(&mut self, limit_offset: S) -> &mut Self {
        self._limit_offset = Some(limit_offset.into());
        self
    }
    pub fn and_where<S: Into<QueryWhere>>(&mut self, query_where: S) -> &mut Self {
        if self._wheres.len() > 0 {
            self._wheres.push(WhereOperator::AND);
        }
        self._wheres.push(WhereOperator::Where(query_where.into()));
        self
    }
    pub fn or_where<S: Into<QueryWhere>>(&mut self, query_where: S) -> &mut Self {
        if self._wheres.len() > 0 {
            self._wheres.push(WhereOperator::OR);
        }
        self._wheres.push(WhereOperator::Where(query_where.into()));
        self
    }
    pub fn and_where_open<S: Into<QueryWhere>>(&mut self) -> &mut Self {
        if self._wheres.len() > 0 {
            self._wheres.push(WhereOperator::AND);
        }
        self._wheres.push(WhereOperator::OPEN);
        self
    }
    pub fn or_where_open<S: Into<QueryWhere>>(&mut self) -> &mut Self {
        if self._wheres.len() > 0 {
            self._wheres.push(WhereOperator::OR);
        }
        self._wheres.push(WhereOperator::OPEN);
        self
    }
    pub fn where_close<S: Into<QueryWhere>>(&mut self) -> &mut Self {
        self._wheres.push(WhereOperator::CLOSE);
        // lets dont panic, if error database will panic
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let qbuild = SqlSelect::new()
            .select("username")
            .select(("user_id", "uid"))
            .table("user")
            .order(("user.created_at", "DESC"))
            .limit_offset((10, 20))
            .build().unwrap();
        assert_eq!(qbuild.query, "SELECT username, user_id AS uid FROM user ORDER BY user.created_at DESC LIMIT 10 OFFSET 20");
    }
}
