use crate::postgres::update_builder::SqlUpdate;
use crate::postgres::insert_builder::SqlInsert;
use crate::postgres::select_builder::SqlSelect;

pub struct PostgresBuilder {
}

impl PostgresBuilder {
    pub fn insert() -> SqlInsert<'static> {
        SqlInsert::new()
    }
    pub fn select() -> SqlSelect<'static> {
        SqlSelect::new()
    }
    pub fn update() -> SqlUpdate<'static> {
        SqlUpdate::new()
    }
}
