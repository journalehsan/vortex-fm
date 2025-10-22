// Helper functions for tab operations
// These are private helpers called from app.rs

use crate::tab::{Tab, Location};

/// Get all tab paths for iteration
pub(crate) fn collect_tab_locations(tabs: &[&Tab]) -> Vec<Location> {
    tabs.iter()
        .map(|tab| tab.location.clone())
        .collect()
}

/// Count total items across tabs
pub(crate) fn count_total_items(tabs: &[&Tab]) -> usize {
    tabs.iter()
        .filter_map(|tab| tab.items_opt().map(|items| items.len()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_tab_locations_empty() {
        let tabs: Vec<&Tab> = vec![];
        let locations = collect_tab_locations(&tabs);
        assert_eq!(locations.len(), 0);
    }

    #[test]
    fn test_count_total_items_empty() {
        let tabs: Vec<&Tab> = vec![];
        let count = count_total_items(&tabs);
        assert_eq!(count, 0);
    }
}
