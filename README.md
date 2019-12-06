# Postgres Query Builder for Rust
- for usage with postgres crate

# For How it works
~~~rs
fn test_postgres_db() -> Result<(), PostgresError> {
    let conn = Connection::connect("postgresql://pguser:password@localhost:54321/postgres_query_builder_postgres", TlsMode::None)
    .unwrap();
    conn.execute("DROP TABLE IF EXISTS person", &[]).unwrap();
    conn.execute("CREATE TABLE person (
        id              INT PRIMARY KEY,
        name            VARCHAR(255) NOT NULL,
        data            Bytea
      )", &[]).unwrap();

    let opt: Option<Vec<u8>> = None;
    let mut sqlinsert = postgres_query_builder::SqlInsert::new();
    let qbuild = sqlinsert
        .table("person")
        .into(("id", 3))
        .into(("name", "Hello World"))
        .into(("data", opt))
        .build().unwrap();
    // println!("{:?}{:?}", qbuild.query, qbuild.parameters);
    conn.execute(
        &qbuild.query,
        &qbuild.parameters,
    )?;
    let mut sqlselect = postgres_query_builder::SqlSelect::new();
    let qbuild2 = sqlselect
        .select("id")
        .select(("name", "person_name"))
        .select("data")
        .table("person")
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

## Limitation
Current queryfmt macro does not intelligently notify error like how format! macro do
