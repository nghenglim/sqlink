use crate::error::Error;

#[derive(Clone, Debug)]
pub struct QueryGroups(Vec<QueryGroup>);

impl Default for QueryGroups {
    fn default() -> Self {
        QueryGroups(Vec::new())
    }
}

impl QueryGroups {
    pub fn build(&self) -> Result<String, Error> {
        Ok(self.0.clone().into_iter().map(|s| s.build()).collect::<Vec<String>>().join(", "))
    }
    pub fn push(&mut self, field: QueryGroup) {
        self.0.push(field);
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Clone, Debug)]
pub struct QueryGroup {
    pub name: String,
}

impl QueryGroup {
    fn build(&self) -> String {
        self.name.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_group_1() {
        let field: QueryGroup = QueryGroup {
            name: "u.id".to_owned(),
        };

        assert_eq!(field.build(), "u.id");
    }
}
