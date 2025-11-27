use serde::{Deserialize, Serialize};

/// Pagination request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    /// Page number (1-based)
    #[serde(default = "default_page")]
    pub page: u64,

    /// Number of items per page
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

fn default_page() -> u64 {
    1
}

fn default_page_size() -> u64 {
    20
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: default_page(),
            page_size: default_page_size(),
        }
    }
}

impl PaginationParams {
    /// Create new pagination params
    pub fn new(page: u64, page_size: u64) -> Self {
        Self { page, page_size }
    }

    /// Validate and normalize pagination params
    pub fn validate(mut self) -> Self {
        if self.page == 0 {
            self.page = 1;
        }
        if self.page_size == 0 {
            self.page_size = default_page_size();
        }
        if self.page_size > 100 {
            self.page_size = 100; // Max page size
        }
        self
    }

    /// Calculate offset for database query (0-based)
    pub fn offset(&self) -> u64 {
        (self.page - 1) * self.page_size
    }

    /// Get limit for database query
    pub fn limit(&self) -> u64 {
        self.page_size
    }
}

/// Paginated response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    /// List of items for current page
    pub items: Vec<T>,

    /// Current page number
    pub page: u64,

    /// Number of items per page
    pub page_size: u64,

    /// Total number of items
    pub total: u64,

    /// Total number of pages
    pub total_pages: u64,

    /// Whether there is a next page
    pub has_next: bool,

    /// Whether there is a previous page
    pub has_prev: bool,
}

impl<T> PaginatedResponse<T> {
    /// Create a new paginated response
    pub fn new(items: Vec<T>, params: &PaginationParams, total: u64) -> Self {
        let total_pages = if total == 0 {
            0
        } else {
            (total + params.page_size - 1) / params.page_size
        };

        Self {
            items,
            page: params.page,
            page_size: params.page_size,
            total,
            total_pages,
            has_next: params.page < total_pages,
            has_prev: params.page > 1,
        }
    }

    /// Map the items to a different type
    pub fn map<U, F>(self, f: F) -> PaginatedResponse<U>
    where
        F: FnMut(T) -> U,
    {
        PaginatedResponse {
            items: self.items.into_iter().map(f).collect(),
            page: self.page,
            page_size: self.page_size,
            total: self.total,
            total_pages: self.total_pages,
            has_next: self.has_next,
            has_prev: self.has_prev,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_params_default() {
        let params = PaginationParams::default();
        assert_eq!(params.page, 1);
        assert_eq!(params.page_size, 20);
    }

    #[test]
    fn test_pagination_params_validate() {
        let params = PaginationParams::new(0, 0).validate();
        assert_eq!(params.page, 1);
        assert_eq!(params.page_size, 20);

        let params = PaginationParams::new(1, 200).validate();
        assert_eq!(params.page_size, 100); // Max
    }

    #[test]
    fn test_pagination_offset() {
        let params = PaginationParams::new(1, 20);
        assert_eq!(params.offset(), 0);

        let params = PaginationParams::new(2, 20);
        assert_eq!(params.offset(), 20);

        let params = PaginationParams::new(3, 10);
        assert_eq!(params.offset(), 20);
    }

    #[test]
    fn test_paginated_response() {
        let items = vec![1, 2, 3];
        let params = PaginationParams::new(2, 3);
        let response = PaginatedResponse::new(items, &params, 10);

        assert_eq!(response.page, 2);
        assert_eq!(response.page_size, 3);
        assert_eq!(response.total, 10);
        assert_eq!(response.total_pages, 4);
        assert!(response.has_next);
        assert!(response.has_prev);
    }
}
