#[macro_export]
macro_rules! fmt_query {
    ($a:literal) => {
        sqlink::format_query_postgres($a.into(), vec![])
    };
    ($a:literal, $($x:tt), *) => {
        sqlink::format_query_postgres($a.into(), vec![$(Box::new($x)), *])
    };
}


#[cfg(test)]
mod tests {
    use sqlink::{PostgresBuilder};
    #[test]
    fn test_insert_builder_derive() {
        let user_spouse: Option<String> = None;
        let mut sql_insert = PostgresBuilder::insert();
        let qbuild = sql_insert
            .table("user")
            .into("spouse", user_spouse)
            .into("age", 1337)
            .into_raw("name", fmt_query!("LOWER({})", "foo"))
            .build().unwrap();
        assert_eq!(qbuild.query, "INSERT INTO \"user\"(\"spouse\",\"age\",\"name\") VALUES ($1,$2,LOWER($3))");
        assert_eq!(format!("{:?}", qbuild.parameters), "[None, 1337, \"foo\"]");
    }
    #[test]
    fn test_select_builder_derive() {
        let mut sql_select = PostgresBuilder::select();
        let qbuild = sql_select
            .select("u.id")
            .select("ub.account_no")
            .and_where(fmt_query!("u.id = {}", 1))
            .table(("user", "u"))
            .left_join(("user_bank", "ub"), fmt_query!("u.user_id = ub.user_id and ub.code = {}", "abc"))
            .build().unwrap();
        assert_eq!(qbuild.query, "SELECT u.id, ub.account_no FROM \"user\" AS u LEFT JOIN \"user_bank\" AS ub ON u.user_id = ub.user_id and ub.code = $1 WHERE u.id = $2");
        assert_eq!(format!("{:?}", qbuild.parameters), "[\"abc\", 1]");
    }
}
