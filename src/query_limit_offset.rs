#[derive(Clone, Debug)]
pub struct QueryLimitOffset {
    limit: u32,
    offset: u32,
}

impl QueryLimitOffset {
    pub fn build(&self) -> String {
        if self.offset == 0 {
            format!("LIMIT {}", self.limit)
        } else {
            format!("LIMIT {} OFFSET {}", self.limit, self.offset)
        }
    }
}

impl From<u32> for QueryLimitOffset {
    fn from(limit: u32) -> Self {
        QueryLimitOffset {
            limit: limit,
            offset: 0,
        }
    }
}

impl From<(u32, u32)> for QueryLimitOffset {
    fn from(tup: (u32, u32)) -> Self {
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

        assert_eq!(limit_offset.build(), "LIMIT 10");
    }

    #[test]
    fn test_limit_offset_2() {
        let limit_offset: QueryLimitOffset = 1.into();
        assert_eq!(limit_offset.build(), "LIMIT 1");
    }

    #[test]
    fn test_limit_offset_3() {
        let limit_offset: QueryLimitOffset = (10, 10).into();
        assert_eq!(limit_offset.build(), "LIMIT 10 OFFSET 10");
    }
}
