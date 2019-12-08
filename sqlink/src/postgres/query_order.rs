use crate::error::Error;

#[derive(Clone, Debug)]
pub struct QueryOrders(Vec<QueryOrder>);

impl Default for QueryOrders {
    fn default() -> Self {
        QueryOrders(Vec::new())
    }
}

impl QueryOrders {
    pub fn build(&self) -> Result<String, Error> {
        Ok(self.0.clone().into_iter().map(|s| s.build()).collect::<Vec<String>>().join(", "))
    }
    pub fn push(&mut self, field: QueryOrder) {
        self.0.push(field);
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Clone, Debug)]
pub struct QueryOrder {
    name: String, // TBD on DESC ASC NULL FIRST CASE WHEN
}

impl QueryOrder {
    fn build(&self) -> String {
        self.name.clone()
    }
}

impl From<&str> for QueryOrder {
    fn from(field: &str) -> Self {
        QueryOrder {
            name: field.to_owned(),
        }
    }
}

impl From<(&str, &str)> for QueryOrder {
    fn from(tup: (&str, &str)) -> Self {
        QueryOrder {
            name: format!("{} {}", tup.0, tup.1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_order_1() {
        let field: QueryOrder = QueryOrder {
            name: "u.id ASC".to_owned(),
        };

        assert_eq!(field.build(), "u.id ASC");
    }

    #[test]
    fn test_order_2() {
        let field: QueryOrder = "u.created_at".into();
        assert_eq!(field.build(), "u.created_at");
    }
}
