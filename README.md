## sqlink [![Latest Version](https://img.shields.io/crates/v/sqlink.svg)](https://crates.io/crates/sqlink)
A simple query builder to use with low level library such as rust-postgres

## For How it works
~~~rs
extern crate postgres;
use postgres::{Connection, TlsMode, Error as PostgresError};
use sqlink::{PostgresBuilder};
use sqlink_derive::{fmt_query};
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
    let conn = Connection::connect("postgresql://pguser:password@localhost:54321/sqlink_postgres", TlsMode::None)
    .unwrap();
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
    let qbuild = sqlinsert
        .table("person")
        .returning("id")
        .into(("id", person_form.id))
        .into(("name", person_form.name))
        .into(("data", person_form.data))
        .build().unwrap();
    conn.execute(
        &qbuild.query,
        &qbuild.parameters,
    )?;
    let mut sqlselect = PostgresBuilder::select();
    let qbuild2 = sqlselect
        .select("id")
        .select(("name", "person_name"))
        .select("data")
        .table("person")
        .and_where(
            fmt_query!("person.id = {}", 3) // note that 3 has to be same type as person id, which is i32/INT here
        )
        .build().unwrap();
    let mut person_vec: Vec<Person> = Vec::new();
    for row in &conn.query(&qbuild2.query, &qbuild2.parameters).unwrap() {
        person_vec.push(Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2),
        })
    }
    assert_eq!(person_vec, vec![Person {
        id: 3,
        name: "Hello World".to_owned(),
        data: None
    }]);
    Ok(())
}

~~~

### Limitation
Current fmt_query macro does not intelligently notify error like how format! macro do
