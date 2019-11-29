#[derive(Clone, Debug)]
pub struct QuerySelectField {
    name: String,
    alias: Option<String>,
}

impl QuerySelectField {
    pub fn build(&self) -> String {
        if let Some(alias) = &self.alias {
            format!("{} AS {}", self.name, alias)
        } else {
            self.name.clone()
        }
    }
}

impl From<&str> for QuerySelectField {
    fn from(field: &str) -> Self {
        QuerySelectField {
            name: field.to_owned(),
            alias: None,
        }
    }
}

impl From<(&str, &str)> for QuerySelectField {
    fn from(fieldandalias: (&str, &str)) -> Self {
        QuerySelectField {
            name: fieldandalias.0.to_owned(),
            alias: Some(fieldandalias.1.to_owned()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_select_field_1() {
        let field: QuerySelectField = QuerySelectField {
            alias: Some("uid".to_owned()),
            name: "u.id".to_owned(),
        };

        assert_eq!(field.build(), "u.id AS uid");
    }

    #[test]
    fn test_select_field_2() {
        let field: QuerySelectField = "user_id".into();
        assert_eq!(field.build(), "user_id");
    }

    #[test]
    fn test_select_field_3() {
        let field: QuerySelectField = ("user_id", "uid").into();
        assert_eq!(field.build(), "user_id AS uid");
    }
}
