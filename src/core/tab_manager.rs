use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use gtk::prelude::*;
use gtk::{Box, Orientation, Button, Label, Notebook};
use crate::core::file_manager::FileManagerState;
use crate::core::config::VortexConfig;
use crate::core::navigation_history::NavigationHistory;

#[derive(Clone)]
pub struct Tab {
    pub id: usize,
    pub navigation_history: NavigationHistory,
    pub title: String,
    pub is_active: bool,
}

impl Tab {
    pub fn new(id: usize, path: PathBuf) -> Self {
        let navigation_history = NavigationHistory::new(path.clone());
        let title = Self::generate_title(&path);
        
        Self {
            id,
            navigation_history,
            title,
            is_active: false,
        }
    }
    
    fn generate_title(path: &PathBuf) -> String {
        // Generate a nice title based on the current path
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            // Capitalize first letter
            let mut chars = file_name.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        } else {
            "Home".to_string()
        }
    }
    
    pub fn update_title(&mut self) {
        self.title = Self::generate_title(self.navigation_history.current());
    }
    
    pub fn current_path(&self) -> &PathBuf {
        self.navigation_history.current()
    }
    
    pub fn navigate_to(&mut self, path: PathBuf) {
        self.navigation_history.navigate_to(path);
        self.update_title();
    }
}

#[derive(Clone)]
pub struct TabManager {
    pub tabs: Vec<Tab>,
    pub active_tab_id: Option<usize>,
    pub next_tab_id: usize,
    pub notebook: Option<Notebook>,
}

impl TabManager {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active_tab_id: None,
            next_tab_id: 0,
            notebook: None,
        }
    }
    
    pub fn add_tab(&mut self, path: PathBuf) -> usize {
        let tab_id = self.next_tab_id;
        self.next_tab_id += 1;
        
        let mut tab = Tab::new(tab_id, path);
        tab.is_active = true;
        
        // Deactivate other tabs
        for t in &mut self.tabs {
            t.is_active = false;
        }
        
        self.tabs.push(tab);
        self.active_tab_id = Some(tab_id);
        
        tab_id
    }
    
    pub fn close_tab(&mut self, tab_id: usize) -> bool {
        // Do not allow closing the last remaining tab
        if self.tabs.len() <= 1 {
            crate::utils::simple_debug::debug_info("TAB_MANAGER", "Refusing to close last tab");
            return false;
        }
        crate::utils::simple_debug::debug_info("TAB_MANAGER", &format!("Request to close tab_id={} (tabs={})", tab_id, self.tabs.len()));
        if let Some(pos) = self.tabs.iter().position(|t| t.id == tab_id) {
            crate::utils::simple_debug::debug_info("TAB_MANAGER", &format!("Found tab at index {}", pos));
            self.tabs.remove(pos);
            
            // If we closed the active tab, activate another one
            if self.active_tab_id == Some(tab_id) {
                if !self.tabs.is_empty() {
                    let new_active = self.tabs.last().unwrap().id;
                    crate::utils::simple_debug::debug_info("TAB_MANAGER", &format!("Activating new tab_id={}", new_active));
                    self.activate_tab(new_active);
                } else {
                    crate::utils::simple_debug::debug_info("TAB_MANAGER", "No tabs left after close; clearing active_tab_id");
                    self.active_tab_id = None;
                }
            }
            
            crate::utils::simple_debug::debug_info("TAB_MANAGER", &format!("Close success; tabs now={}", self.tabs.len()));
            true
        } else {
            crate::utils::simple_debug::debug_info("TAB_MANAGER", &format!("Tab id {} not found; close failed", tab_id));
            false
        }
    }
    
    pub fn activate_tab(&mut self, tab_id: usize) -> Option<PathBuf> {
        for tab in &mut self.tabs {
            tab.is_active = tab.id == tab_id;
        }
        self.active_tab_id = Some(tab_id);
        
        // Return the current path of the activated tab
        self.get_active_tab().map(|t| t.current_path().clone())
    }
    
    pub fn navigate_active_tab_to(&mut self, path: PathBuf) {
        if let Some(active_id) = self.active_tab_id {
            if let Some(tab) = self.tabs.iter_mut().find(|t| t.id == active_id) {
                tab.navigate_to(path);
            }
        }
    }
    
    pub fn get_active_tab(&self) -> Option<&Tab> {
        self.active_tab_id.and_then(|id| self.tabs.iter().find(|t| t.id == id))
    }
    
    pub fn get_active_path(&self) -> Option<PathBuf> {
        self.get_active_tab().map(|t| t.current_path().clone())
    }
    
    pub fn update_active_tab_title(&mut self) {
        if let Some(active_id) = self.active_tab_id {
            if let Some(tab) = self.tabs.iter_mut().find(|t| t.id == active_id) {
                tab.update_title();
            }
        }
    }
}
