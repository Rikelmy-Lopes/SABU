//! Analytics cache shared by the UI components.

mod cached_analytics;
mod table_cache;

pub(crate) use cached_analytics::AnalyticsCache;
pub(crate) use table_cache::TableCache;

#[cfg(test)]
#[path = "analytics_cache_tests.rs"]
mod analytics_cache_tests;
