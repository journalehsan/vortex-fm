use std::path::PathBuf;
use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct SelectionManager {
    pub selected_files: HashSet<PathBuf>,
    pub primary_selection: Option<PathBuf>, // The main selected file for details panel
}

impl SelectionManager {
    pub fn new() -> Self {
        Self {
            selected_files: HashSet::new(),
            primary_selection: None,
        }
    }
    
    pub fn select_file(&mut self, path: PathBuf) {
        self.selected_files.insert(path.clone());
        self.primary_selection = Some(path);
    }
    
    pub fn deselect_file(&mut self, path: &PathBuf) {
        self.selected_files.remove(path);
        if self.primary_selection.as_ref() == Some(path) {
            self.primary_selection = self.selected_files.iter().next().cloned();
        }
    }
    
    pub fn clear_selection(&mut self) {
        self.selected_files.clear();
        self.primary_selection = None;
    }
    
    pub fn is_selected(&self, path: &PathBuf) -> bool {
        self.selected_files.contains(path)
    }
    
    pub fn get_primary_selection(&self) -> Option<&PathBuf> {
        self.primary_selection.as_ref()
    }
    
    pub fn get_selected_count(&self) -> usize {
        self.selected_files.len()
    }
    
    pub fn toggle_selection(&mut self, path: PathBuf) {
        if self.is_selected(&path) {
            self.deselect_file(&path);
        } else {
            self.select_file(path);
        }
    }
    
    pub fn select_all(&mut self, files: Vec<PathBuf>) {
        self.clear_selection();
        for file in &files {
            self.selected_files.insert(file.clone());
        }
        if let Some(first_file) = files.first() {
            self.primary_selection = Some(first_file.clone());
        }
    }
}

// Global selection manager
static mut GLOBAL_SELECTION_MANAGER: Option<Rc<RefCell<SelectionManager>>> = None;

pub fn set_global_selection_manager(selection_manager: Rc<RefCell<SelectionManager>>) {
    unsafe {
        GLOBAL_SELECTION_MANAGER = Some(selection_manager);
    }
}

pub fn get_global_selection_manager() -> Option<Rc<RefCell<SelectionManager>>> {
    unsafe {
        GLOBAL_SELECTION_MANAGER.as_ref().map(|manager| manager.clone())
    }
}

pub fn select_file(path: PathBuf) {
    if let Some(manager_rc) = get_global_selection_manager() {
        manager_rc.borrow_mut().select_file(path);
        // Update details panel
        if let Some(primary) = manager_rc.borrow().get_primary_selection() {
            crate::widgets::details_panel::update_global_details_panel(Some(primary));
        }
        // Refresh the file list to update selection styling
        refresh_file_list();
    }
}

fn refresh_file_list() {
    if let Some(state_rc) = crate::core::navigation::get_global_state() {
        state_rc.borrow().refresh_ui();
    }
}

pub fn clear_selection() {
    if let Some(manager_rc) = get_global_selection_manager() {
        manager_rc.borrow_mut().clear_selection();
        // Update details panel to show folder info
        crate::widgets::details_panel::update_global_details_panel(None);
        // Refresh the file list to update selection styling
        refresh_file_list();
    }
}
