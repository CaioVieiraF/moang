use serde::Deserialize;

/// Number of items returned by each paginated endpoint.
pub const PAGE_SIZE: i64 = 20;

/// Shared `?page=N` query for paginated endpoints.
#[derive(Deserialize)]
pub struct PageQuery {
    pub page: Option<i64>,
}

impl PageQuery {
    /// One-based page number. Missing or non-positive values mean the first page.
    pub fn page(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn offset(&self) -> i64 {
        (self.page() - 1) * PAGE_SIZE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_the_first_page() {
        let query = PageQuery { page: None };

        assert_eq!(query.page(), 1);
        assert_eq!(query.offset(), 0);
    }

    #[test]
    fn clamps_non_positive_pages() {
        let query = PageQuery { page: Some(0) };

        assert_eq!(query.page(), 1);
        assert_eq!(query.offset(), 0);
    }

    #[test]
    fn computes_the_offset_for_later_pages() {
        let query = PageQuery { page: Some(3) };

        assert_eq!(query.page(), 3);
        assert_eq!(query.offset(), 2 * PAGE_SIZE);
    }
}
