use crate::query_field::FieldValue;

#[derive(Clone, Debug)]
pub enum WhereOperator {
    Where(QueryWhere),
    AND,
    OR,
    OPEN,
    CLOSE,
}

impl WhereOperator {
    pub fn build(&self) -> String {
        match self {
            WhereOperator::Where(b) => { b.build() },
            WhereOperator::AND => { String::from("AND") },
            WhereOperator::OR => { String::from("OR") },
            WhereOperator::OPEN => { String::from("(") },
            WhereOperator::CLOSE => { String::from(")") },
        }
    }
}

#[derive(Clone, Debug)]
pub struct QueryWhere {
    name: String,
    parameters: Vec<FieldValue>
}

impl QueryWhere {
    pub fn build(&self) -> String {
        self.name.clone()
    }
}

impl From<&str> for QueryWhere {
    fn from(field: &str) -> Self {
        QueryWhere {
            name: field.to_owned(),
            parameters: Vec::new()
        }
    }
}

impl From<(&str, Vec<FieldValue>)> for QueryWhere {
    fn from(tup: (&str, Vec<FieldValue>)) -> Self {
        QueryWhere {
            name: tup.0.to_owned(),
            parameters: tup.1.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_where_1() {
        let qwhere: QueryWhere = "user.id = 1".into();
        assert_eq!(qwhere.build(), "user.id = 1");
    }
}
