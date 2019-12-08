use crate::error::Error;
use crate::postgres::query_field::{QueryWithParamsLoc};
use crate::postgres::query_where::{QueryWheres, WhereOperator};
use crate::postgres::static_constant::TABLE_ESCAPE;

#[derive(Debug)]
pub struct QueryTables(Vec<QueryTable>);

impl Default for QueryTables {
    fn default() -> Self {
        QueryTables(Vec::new())
    }
}

impl QueryTables {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn build(&self, i: &mut i8) -> Result<QueryWithParamsLoc, Error> {
        if self.0.len() != 1 {
            return Err(Error::Syntax("Table can only select 1 time".into()));
        }
        let mut v: Vec<String> = Vec::new();
        let mut q: Vec<usize> = Vec::new();
        for join in &self.0 {
            let built = join.build(i)?;
            v.push(built.query);
            q.extend(built.parameters_loc);
        }
        Ok(QueryWithParamsLoc {
            query: v.join(","),
            parameters_loc: q,
        })
    }
    pub fn push(&mut self, field: QueryTable) {
        self.0.push(field);
    }
    pub fn get_last_table(&mut self) -> &mut QueryTable {
        let len = self.0.len();
        if len == 0 {
            panic!("cannot select without base table");
        }
        &mut self.0[len - 1]
    }
    pub fn inner_join(&mut self, query_table: QueryTable) -> &mut Self {
        let table = self.get_last_table();
        table.table_join.push(TableJoin::InnerJoin(query_table, QueryWheres::default()));
        self
    }
    pub fn left_join(&mut self, query_table: QueryTable) -> &mut Self {
        let table = self.get_last_table();
        table.table_join.push(TableJoin::LeftJoin(query_table, QueryWheres::default()));
        self
    }
    pub fn right_join(&mut self, query_table: QueryTable) -> &mut Self {
        let table = self.get_last_table();
        table.table_join.push(TableJoin::RightJoin(query_table, QueryWheres::default()));
        self
    }
    pub fn full_join(&mut self, query_table: QueryTable) -> &mut Self {
        let table = self.get_last_table();
        table.table_join.push(TableJoin::FullJoin(query_table, QueryWheres::default()));
        self
    }
    pub fn on(&mut self, query_where: QueryWheres) -> &mut Self {
        let table = self.get_last_table();
        let len = table.table_join.len();
        if len == 0 {
            panic!("cannot on without join table");
        }
        let w = match &mut table.table_join[len - 1] {
            TableJoin::InnerJoin(_, w) => w,
            TableJoin::LeftJoin(_, w) => w,
            TableJoin::RightJoin(_, w) => w,
            TableJoin::FullJoin(_, w) => w,
        };
        if w.len() != 0 {
            w.push(WhereOperator::And);
        }
        w.extend(query_where);
        self
    }
}

#[derive(Debug)]
pub enum TableJoin {
    InnerJoin(QueryTable, QueryWheres),
    LeftJoin(QueryTable, QueryWheres),
    RightJoin(QueryTable, QueryWheres),
    FullJoin(QueryTable, QueryWheres),
}

#[derive(Debug)]
pub struct QueryTable {
    alias: Option<String>,
    name: String,
    schema: Option<String>,
    table_join: Vec<TableJoin>,
}

impl QueryTable {
    fn get_table_name(&self) -> String {
        let escaped_table_name = if let Some(schema) = &self.schema {
            format!("{}{}{}.{}{}{}", TABLE_ESCAPE, schema, TABLE_ESCAPE, TABLE_ESCAPE, self.name, TABLE_ESCAPE)
        } else {
            format!("{}{}{}", TABLE_ESCAPE, self.name.clone(), TABLE_ESCAPE)
        };
        if let Some(alias) = &self.alias {
            format!("{} AS {}", escaped_table_name, alias)
        } else {
            escaped_table_name
        }
    }
    fn build(&self, i: &mut i8) -> Result<QueryWithParamsLoc, Error> {
        let mut v: Vec<String> = Vec::new();
        let mut p: Vec<usize> = Vec::new();
        v.push(self.get_table_name());
        for join in &self.table_join {
            let (op, join_table, qw) = match join {
                TableJoin::InnerJoin(join_table, qw) => {("INNER JOIN", join_table, qw)},
                TableJoin::LeftJoin(join_table, qw) => {("LEFT JOIN", join_table, qw)},
                TableJoin::RightJoin(join_table, qw) => {("RIGHT JOIN", join_table, qw)},
                TableJoin::FullJoin(join_table, qw) => {("FULL JOIN", join_table, qw)},
            };
            v.push(op.to_owned());
            v.push(join_table.get_table_name());
            v.push("ON".to_owned());
            let qwresult = qw.build(i)?;
            v.push(qwresult.query);
            p.extend(qwresult.parameters_loc);
        }
        Ok(QueryWithParamsLoc {
            query: v.join(" "),
            parameters_loc: p,
        })
    }
}
impl From<&str> for QueryTable {
    fn from(table: &str) -> Self {
        QueryTable {
            alias: None,
            name: table.to_owned(),
            schema: None,
            table_join: Vec::new(),
        }
    }
}

impl From<(&str, &str)> for QueryTable {
    fn from(nameandalias: (&str, &str)) -> Self {
        QueryTable {
            alias: Some(nameandalias.1.to_owned()),
            name: nameandalias.0.to_owned(),
            schema: None,
            table_join: Vec::new(),
        }
    }
}

impl From<(&str, &str, &str)> for QueryTable {
    fn from(schemaxnamexalias: (&str, &str, &str)) -> Self {
        QueryTable {
            alias: Some(schemaxnamexalias.2.to_owned()),
            name: schemaxnamexalias.1.to_owned(),
            schema: Some(schemaxnamexalias.0.to_owned()),
            table_join: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_table_1() {
        let table: QueryTable = QueryTable {
            alias: Some("u".to_owned()),
            name: "user".to_owned(),
            schema: Some("public".to_owned()),
            table_join: Vec::new(),
        };

        assert_eq!(table.build(&mut 0).unwrap().query, "\"public\".\"user\" AS u");
    }

    #[test]
    fn test_table_2() {
        let table: QueryTable = "user".into();
        assert_eq!(table.build(&mut 0).unwrap().query, "\"user\"");
    }

    #[test]
    fn test_table_3() {
        let table: QueryTable = ("user", "u").into();
        assert_eq!(table.build(&mut 0).unwrap().query, "\"user\" AS u");
    }
}
