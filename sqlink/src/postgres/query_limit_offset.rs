use crate::error::Error;

#[derive(Clone, Debug)]
pub struct QueryLimitOffset {
    limit: usize,
    offset: usize,
}

impl QueryLimitOffset {
    pub fn build(&self) -> Result<String, Error> {
        if self.offset == 0 {
            Ok(format!("LIMIT {}", self.limit))
        } else {
            Ok(format!("LIMIT {} OFFSET {}", self.limit, self.offset))
        }
    }
}

impl From<usize> for QueryLimitOffset {
    fn from(limit: usize) -> Self {
        QueryLimitOffset {
            limit: limit,
            offset: 0,
        }
    }
}

impl From<(usize, usize)> for QueryLimitOffset {
    fn from(tup: (usize, usize)) -> Self {
        QueryLimitOffset {
            limit: tup.0,
            offset: tup.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_limit_offset_1() {
        let limit_offset: QueryLimitOffset = QueryLimitOffset {
            limit: 10,
            offset: 0,
        };

        assert_eq!(limit_offset.build().unwrap(), "LIMIT 10");
    }

    #[test]
    fn test_limit_offset_2() {
        let limit_offset: QueryLimitOffset = (1usize).into();
        assert_eq!(limit_offset.build().unwrap(), "LIMIT 1");
    }

    #[test]
    fn test_limit_offset_3() {
        let limit_offset: QueryLimitOffset = (10, 10).into();
        assert_eq!(limit_offset.build().unwrap(), "LIMIT 10 OFFSET 10");
    }
}
