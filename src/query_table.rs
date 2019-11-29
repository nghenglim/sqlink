#[derive(Clone, Debug)]
pub struct QueryTable {
    alias: Option<String>,
    name: String,
    schema: Option<String>,
}

impl QueryTable {
    pub fn build(&self) -> String {
        let table_name = if let Some(schema) = &self.schema {
            format!("{}.{}", schema, self.name)
        } else {
            self.name.clone()
        };
        if let Some(alias) = &self.alias {
            format!("{} AS {}", table_name, alias)
        } else {
            table_name
        }
    }
}
impl From<&str> for QueryTable {
    fn from(table: &str) -> Self {
        QueryTable {
            alias: None,
            name: table.to_owned(),
            schema: None,
        }
    }
}

impl From<(&str, &str)> for QueryTable {
    fn from(nameandalias: (&str, &str)) -> Self {
        QueryTable {
            alias: Some(nameandalias.1.to_owned()),
            name: nameandalias.0.to_owned(),
            schema: None,
        }
    }
}

impl From<(&str, &str, &str)> for QueryTable {
    fn from(schemaxnamexalias: (&str, &str, &str)) -> Self {
        QueryTable {
            alias: Some(schemaxnamexalias.2.to_owned()),
            name: schemaxnamexalias.1.to_owned(),
            schema: Some(schemaxnamexalias.0.to_owned()),
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
        };

        assert_eq!(table.build(), "public.user AS u");
    }

    #[test]
    fn test_table_2() {
        let table: QueryTable = "user".into();
        assert_eq!(table.build(), "user");
    }

    #[test]
    fn test_table_3() {
        let table: QueryTable = ("user", "u").into();
        assert_eq!(table.build(), "user AS u");
    }
}
