use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use gtk::prelude::*;
use gtk::{Box, Orientation, Button, Label, Notebook};
use crate::core::file_manager::FileManagerState;
use crate::core::config::VortexConfig;

#[derive(Clone)]
pub struct Tab {
    pub id: usize,
    pub path: PathBuf,
    pub title: String,
    pub is_active: bool,
}

impl Tab {
    pub fn new(id: usize, path: PathBuf) -> Self {
        let title = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string();
        
        Self {
            id,
            path,
            title,
            is_active: false,
        }
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
        if let Some(pos) = self.tabs.iter().position(|t| t.id == tab_id) {
            self.tabs.remove(pos);
            
            // If we closed the active tab, activate another one
            if self.active_tab_id == Some(tab_id) {
                if !self.tabs.is_empty() {
                    let new_active = self.tabs.last().unwrap().id;
                    self.activate_tab(new_active);
                } else {
                    self.active_tab_id = None;
                }
            }
            
            true
        } else {
            false
        }
    }
    
    pub fn activate_tab(&mut self, tab_id: usize) {
        for tab in &mut self.tabs {
            tab.is_active = tab.id == tab_id;
        }
        self.active_tab_id = Some(tab_id);
    }
    
    pub fn get_active_tab(&self) -> Option<&Tab> {
        self.active_tab_id.and_then(|id| self.tabs.iter().find(|t| t.id == id))
    }
    
    pub fn get_active_path(&self) -> Option<PathBuf> {
        self.get_active_tab().map(|t| t.path.clone())
    }
}
