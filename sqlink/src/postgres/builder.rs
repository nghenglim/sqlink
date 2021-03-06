use crate::postgres::update_builder::SqlUpdate;
use crate::postgres::insert_builder::SqlInsert;
use crate::postgres::select_builder::SqlSelect;
use crate::postgres::delete_builder::SqlDelete;

/// examples
/// ```no_run
/// use sqlink::{
///     PostgresBuilder,
///     postgres::{format_query, op}
/// };
/// struct Person {
///     id: i32,
///     name: String,
///     data: Option<Vec<u8>>
/// }
///
/// struct PersonForm {
///     id: i32,
///     name: String,
///     data: Option<Vec<u8>>
/// }

/// // let mut conn = Client::connect("postgresql://pguser:password@localhost:54321/sqlink_postgres", NoTls).unwrap();
/// let person_form = PersonForm {
///     id: 3,
///     name: "Hello World".to_owned(),
///     data: None
/// };
/// let mut sqlinsert = PostgresBuilder::insert();
/// let qbuiltinsert = sqlinsert
///     .table("person")
///     .returning("id")
///     .set("id", &person_form.id)
///     .set("name", &person_form.name)
///     .set("data", &person_form.data)
///     .build().unwrap();
/// // let mut id: i32 = 0;
/// // for row in &conn.query(
/// //     qbuiltinsert.query.as_str(),
/// //     &qbuiltinsert.parameters,
/// // )? {
/// //     id = row.get(0);
/// // }
/// // assert_eq!(id, 3);

/// let mut sqlupdate = PostgresBuilder::update();
/// let qbuiltinsert = sqlupdate
///     .table("person")
///     .set("name", &("Real Hello World"))
///     .and_where(
///         op::eq("person.id", &3) // equivalence to format_query("person.id = {}", vec![&3]), note that 3 has to be same type as person id, which is i32/INT here
///     )
///     .build().unwrap();
/// let mut sqlselect = PostgresBuilder::select();
/// let qbuiltselect = sqlselect
///     .select("id")
///     .select_as("name", "person_name")
///     .select("data")
///     .table("person")
///     .and_where(
///         format_query("person.id = {}", vec![&3])
///     )
///     .limit_offset(10) // equivalent of limit_offset((10, 0)), which is limit 10 offset 0
///     .order("id", "ASC")
///     .build().unwrap();
/// // misc feature
/// sqlselect
///     .reset_selects()
///     .select("COUNT(id)")
///     .group("something")
///     .build().unwrap();
///
/// let mut sqldelete = PostgresBuilder::delete();
/// let qbuiltdelete = sqldelete
///     .table("person")
///     .and_where(
///         op::eq("person.id", &3)
///     )
///     .build().unwrap();
/// ```
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
    pub fn delete() -> SqlDelete<'static> {
        SqlDelete::new()
    }
}
