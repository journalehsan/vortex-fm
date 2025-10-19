use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use crate::core::file_manager::FileManagerState;

// Global state for navigation
static mut GLOBAL_STATE: Option<Rc<RefCell<FileManagerState>>> = None;

pub fn set_global_state(state: Rc<RefCell<FileManagerState>>) {
    unsafe {
        GLOBAL_STATE = Some(state);
    }
}

pub fn get_global_state() -> Option<Rc<RefCell<FileManagerState>>> {
    unsafe {
        GLOBAL_STATE.as_ref().map(|state| state.clone())
    }
}

pub fn navigate_to_directory(path: PathBuf) {
    if let Some(state_rc) = get_global_state() {
        let mut state = state_rc.borrow_mut();
        state.navigate_to(path);
        state.refresh_ui();
    }
}
