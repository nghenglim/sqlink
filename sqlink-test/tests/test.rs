extern crate postgres;
use postgres::{Client, NoTls, Error as PostgresError};
use sqlink::{
    PostgresBuilder,
    postgres::{format_query, op}
};
#[derive(Debug, PartialEq)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}

struct PersonForm {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}

#[test]
fn test_postgres_db() -> Result<(), PostgresError> {
    let mut conn = Client::connect("postgresql://pguser:password@localhost:54321/sqlink_postgres", NoTls).unwrap();
    conn.execute("DROP TABLE IF EXISTS person", &[]).unwrap();
    conn.execute("CREATE TABLE person (
        id              INT PRIMARY KEY,
        name            VARCHAR(255) NOT NULL,
        data            Bytea
      )", &[]).unwrap();

    let person_form = PersonForm {
        id: 3,
        name: "Hello World".to_owned(),
        data: None
    };
    let mut sqlinsert = PostgresBuilder::insert();
    let qbuiltinsert = sqlinsert
        .table("person")
        .returning("id")
        .set("id", &person_form.id)
        .set("name", &person_form.name)
        .set("data", &person_form.data)
        .build().unwrap();
    let mut id: i32 = 0;
    for row in &conn.query(
        qbuiltinsert.query.as_str(),
        &qbuiltinsert.parameters,
    )? {
        id = row.get(0);
    }
    assert_eq!(id, 3);

    let mut sqlupdate = PostgresBuilder::update();
    let qbuiltinsert = sqlupdate
        .table("person")
        .set("name", &("Real Hello World"))
        .and_where(
            op::eq("person.id", &3) // equivalence to format_query("person.id = {}", vec![&3]), note that 3 has to be same type as person id, which is i32/INT here
        )
        .build().unwrap();
    conn.query(
        qbuiltinsert.query.as_str(),
        &qbuiltinsert.parameters,
    )?;
    let mut sqlselect = PostgresBuilder::select();
    let qbuiltselect = sqlselect
        .select("id")
        .select_as("name", "person_name")
        .select("data")
        .table("person")
        .and_where(
            format_query("person.id = {}", vec![&3])
        )
        .limit_offset(10) // equivalent of limit_offset((10, 0)), which is limit 10 offset 0
        .order("id", "ASC")
        .build().unwrap();
    let mut person_vec: Vec<Person> = Vec::new();
    for row in &conn.query(qbuiltselect.query.as_str(), &qbuiltselect.parameters).unwrap() {
        person_vec.push(Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2),
        })
    }
    // // misc feature
    // sqlselect
    //     .reset_selects()
    //     .select("COUNT(id)")
    //     .group("something")
    //     .build().unwrap();
    assert_eq!(person_vec, vec![Person {
        id: 3,
        name: "Real Hello World".to_owned(),
        data: None
    }]);
    let mut sqldelete = PostgresBuilder::delete();
    let qbuiltdelete = sqldelete
        .table("person")
        .and_where(
            op::eq("person.id", &3)
        )
        .build().unwrap();
    conn.query(
        qbuiltdelete.query.as_str(),
        &qbuiltdelete.parameters,
    )?;

    let rows = &conn.query(qbuiltselect.query.as_str(), &qbuiltselect.parameters).unwrap();
    assert_eq!(rows.len(), 0);
    Ok(())
}
